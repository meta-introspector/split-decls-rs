// Generated macro for zero_low_word (function)
macro_rules! Depcrate_math_k_tanzero_low_word {
() => {
// Module: crate::math::k_tan
// Provides: {"zero_low_word"}
// Dependencies: {}
fn zero_low_word (x : f64) -> f64 { f64 :: from_bits (f64 :: to_bits (x) & 0xFFFF_FFFF_0000_0000) }
};
}
