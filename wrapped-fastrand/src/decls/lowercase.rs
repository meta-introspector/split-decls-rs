macro_rules! lowercase {
    () => {
        # [doc = " Generates a random `char` in range a-z."] # [inline] pub fn lowercase () -> char { with_rng (| r | r . lowercase ()) }
    };
}

lowercase!();