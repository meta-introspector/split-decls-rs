// Generated macro for expo2 (function)
macro_rules! Depcrate_math_expo2expo2 {
() => {
// Module: crate::math::expo2
// Provides: {"expo2"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn expo2 (x : f64) -> f64 { const K : i32 = 2043 ; let kln2 = f64 :: from_bits (0x40962066151add8b) ; let scale = combine_words (((0x3ff + K / 2) as u32) << 20 , 0) ; exp (x - kln2) * scale * scale }
};
}
