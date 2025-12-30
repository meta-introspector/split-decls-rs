// Generated macro for get_high_word (function)
macro_rules! Depcrate_libmget_high_word {
() => {
// Module: crate::libm
// Provides: {"get_high_word"}
// Dependencies: {}
# [inline] fn get_high_word (x : f64) -> u32 { (x . to_bits () >> 32) as u32 }
};
}
