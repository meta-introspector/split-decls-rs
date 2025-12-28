macro_rules! alphabetic {
    () => {
        # [doc = " Generates a random `char` in ranges a-z and A-Z."] # [inline] pub fn alphabetic () -> char { with_rng (| r | r . alphabetic ()) }
    };
}

alphabetic!();