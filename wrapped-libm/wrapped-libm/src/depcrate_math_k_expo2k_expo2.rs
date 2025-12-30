// Generated macro for k_expo2 (function)
macro_rules! Depcrate_math_k_expo2k_expo2 {
() => {
// Module: crate::math::k_expo2
// Provides: {"k_expo2"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_expo2 (x : f64) -> f64 { let k_ln2 = f64 :: from_bits (0x40962066151add8b) ; let scale = f64 :: from_bits (((((0x3ff + K / 2) as u32) << 20) as u64) << 32) ; exp (x - k_ln2) * scale * scale }
};
}
