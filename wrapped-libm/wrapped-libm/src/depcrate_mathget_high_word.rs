// Generated macro for get_high_word (function)
macro_rules! Depcrate_mathget_high_word {
() => {
// Module: crate::math
// Provides: {"get_high_word"}
// Dependencies: {}
# [inline] fn get_high_word (x : f64) -> u32 { (x . to_bits () >> 32) as u32 }
};
}
