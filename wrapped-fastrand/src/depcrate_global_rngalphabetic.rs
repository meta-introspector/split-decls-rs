// Generated macro for alphabetic (function)
macro_rules! Depcrate_global_rngalphabetic {
() => {
// Module: crate::global_rng
// Provides: {"alphabetic"}
// Dependencies: {}
# [doc = " Generates a random `char` in ranges a-z and A-Z."] # [inline] pub fn alphabetic () -> char { with_rng (| r | r . alphabetic ()) }
};
}
