// Generated macro for uppercase (function)
macro_rules! Depcrate_global_rnguppercase {
() => {
// Module: crate::global_rng
// Provides: {"uppercase"}
// Dependencies: {}
# [doc = " Generates a random `char` in range A-Z."] # [inline] pub fn uppercase () -> char { with_rng (| r | r . uppercase ()) }
};
}
