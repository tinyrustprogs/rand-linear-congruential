struct Rand {
    state: u32,
}
#[derive(Debug)]
struct Interval {
    low: usize,
    high: usize,
}
// linear congruential method
impl Rand {
    const A: u32 = 5u32; // must be % 4 == 1 and < u32::MAX
    const C: u32 = 3u32; // must be odd      and < u32::MAX
    fn new(seed: u32) -> Rand {
        return Rand { state: seed };
    }
    fn gen(&mut self, i: &Interval) -> u32 {
        self.state = self.state.wrapping_mul(Rand::A).wrapping_add(Rand::C);
        return (self.state % (i.high - i.low) as u32) + i.low as u32;
    }
}

//fn next(x0: u8) -> u8 {
//    let a = 101;
//    let c = 137;
//    return x0.wrapping_mul(a).wrapping_add(c); // should overflow automatically, no modulo needed
//}

fn main() {
    //let mut xn = 0;
    //for _ in 0..256 {
    //    xn = next(xn);
    //    println!("{:#?}", xn);
    //}
    let mut rnd = Rand::new(0x13371337);
    for _ in 0..256 {
        let n = rnd.gen(&Interval {
            low: 0,
            high: 256,
        });
        println!("{:#?}", n);
    }
}
