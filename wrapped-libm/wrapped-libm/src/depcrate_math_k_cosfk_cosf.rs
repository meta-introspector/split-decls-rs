// Generated macro for k_cosf (function)
macro_rules! Depcrate_math_k_cosfk_cosf {
() => {
// Module: crate::math::k_cosf
// Provides: {"k_cosf"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_cosf (x : f64) -> f32 { let z = x * x ; let w = z * z ; let r = C2 + z * C3 ; (((1.0 + z * C0) + w * C1) + (w * z) * r) as f32 }
};
}
