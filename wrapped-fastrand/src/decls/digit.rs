macro_rules! digit {
    () => {
        # [doc = " Generates a random digit in the given `base`."] # [doc = ""] # [doc = " Digits are represented by `char`s in ranges 0-9 and a-z."] # [doc = ""] # [doc = " Panics if the base is zero or greater than 36."] # [inline] pub fn digit (base : u32) -> char { with_rng (| r | r . digit (base)) }
    };
}

digit!();