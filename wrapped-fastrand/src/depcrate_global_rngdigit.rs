// Generated macro for digit (function)
macro_rules! Depcrate_global_rngdigit {
() => {
// Module: crate::global_rng
// Provides: {"digit"}
// Dependencies: {}
# [doc = " Generates a random digit in the given `base`."] # [doc = ""] # [doc = " Digits are represented by `char`s in ranges 0-9 and a-z."] # [doc = ""] # [doc = " Panics if the base is zero or greater than 36."] # [inline] pub fn digit (base : u32) -> char { with_rng (| r | r . digit (base)) }
};
}
