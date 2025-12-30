// Generated macro for k_cos (function)
macro_rules! Depcrate_math_k_cosk_cos {
() => {
// Module: crate::math::k_cos
// Provides: {"k_cos"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_cos (x : f64 , y : f64) -> f64 { let z = x * x ; let w = z * z ; let r = z * (C1 + z * (C2 + z * C3)) + w * w * (C4 + z * (C5 + z * C6)) ; let hz = 0.5 * z ; let w = 1.0 - hz ; w + (((1.0 - w) - hz) + (z * r - x * y)) }
};
}
