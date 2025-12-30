// Generated macro for k_sinf (function)
macro_rules! Depcrate_math_k_sinfk_sinf {
() => {
// Module: crate::math::k_sinf
// Provides: {"k_sinf"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_sinf (x : f64) -> f32 { let z = x * x ; let w = z * z ; let r = S3 + z * S4 ; let s = z * x ; ((x + s * (S1 + z * S2)) + s * w * r) as f32 }
};
}
