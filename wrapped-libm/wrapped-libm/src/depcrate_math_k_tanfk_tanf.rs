// Generated macro for k_tanf (function)
macro_rules! Depcrate_math_k_tanfk_tanf {
() => {
// Module: crate::math::k_tanf
// Provides: {"k_tanf"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_tanf (x : f64 , odd : bool) -> f32 { let z = x * x ; let mut r = T [4] + z * T [5] ; let t = T [2] + z * T [3] ; let w = z * z ; let s = z * x ; let u = T [0] + z * T [1] ; r = (x + s * u) + (s * w) * (t + w * r) ; (if odd { - 1. / r } else { r }) as f32 }
};
}
