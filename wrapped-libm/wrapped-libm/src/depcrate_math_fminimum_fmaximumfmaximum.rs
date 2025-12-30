// Generated macro for fmaximum (function)
macro_rules! Depcrate_math_fminimum_fmaximumfmaximum {
() => {
// Module: crate::math::fminimum_fmaximum
// Provides: {"fmaximum"}
// Dependencies: {}
# [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum (x : f64 , y : f64) -> f64 { super :: generic :: fmaximum (x , y) }
};
}
