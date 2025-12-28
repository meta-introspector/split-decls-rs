macro_rules! f32 {
    () => {
        # [doc = " Generates a random `f32` in range `0..1`."] pub fn f32 () -> f32 { with_rng (| r | r . f32 ()) }
    };
}

f32!();