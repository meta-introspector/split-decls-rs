macro_rules! alphanumeric {
    () => {
        # [doc = " Generates a random `char` in ranges a-z, A-Z and 0-9."] # [inline] pub fn alphanumeric () -> char { with_rng (| r | r . alphanumeric ()) }
    };
}

alphanumeric!();