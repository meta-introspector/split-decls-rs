macro_rules! f64_inclusive {
    () => {
        # [doc = " Generates a random `f64` in range `0..=1`."] pub fn f64_inclusive () -> f64 { with_rng (| r | r . f64_inclusive ()) }
    };
}

f64_inclusive!();