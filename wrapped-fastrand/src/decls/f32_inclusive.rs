macro_rules! f32_inclusive {
    () => {
        # [doc = " Generates a random `f32` in range `0..=1`."] pub fn f32_inclusive () -> f32 { with_rng (| r | r . f32_inclusive ()) }
    };
}

f32_inclusive!()