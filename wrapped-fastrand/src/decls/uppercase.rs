macro_rules! uppercase {
    () => {
        # [doc = " Generates a random `char` in range A-Z."] # [inline] pub fn uppercase () -> char { with_rng (| r | r . uppercase ()) }
    };
}

uppercase!();