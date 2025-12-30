// Generated macro for k_sin (function)
macro_rules! Depcrate_math_k_sink_sin {
() => {
// Module: crate::math::k_sin
// Provides: {"k_sin"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_sin (x : f64 , y : f64 , iy : i32) -> f64 { let z = x * x ; let w = z * z ; let r = S2 + z * (S3 + z * S4) + z * w * (S5 + z * S6) ; let v = z * x ; if iy == 0 { x + v * (S1 + z * r) } else { x - ((z * (0.5 * y - v * r) - y) - v * S1) } }
};
}
