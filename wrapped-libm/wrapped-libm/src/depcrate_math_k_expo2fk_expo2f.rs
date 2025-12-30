// Generated macro for k_expo2f (function)
macro_rules! Depcrate_math_k_expo2fk_expo2f {
() => {
// Module: crate::math::k_expo2f
// Provides: {"k_expo2f"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_expo2f (x : f32) -> f32 { let k_ln2 = f32 :: from_bits (0x4322e3bc) ; let scale = f32 :: from_bits (((0x7f + K / 2) as u32) << 23) ; expf (x - k_ln2) * scale * scale }
};
}
