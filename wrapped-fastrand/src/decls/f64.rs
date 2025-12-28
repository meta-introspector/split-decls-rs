macro_rules! f64 {
    () => {
        # [doc = " Generates a random `f64` in range `0..1`."] pub fn f64 () -> f64 { with_rng (| r | r . f64 ()) }
    };
}

f64!()