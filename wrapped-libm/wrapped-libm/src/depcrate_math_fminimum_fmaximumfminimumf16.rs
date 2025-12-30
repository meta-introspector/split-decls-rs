// Generated macro for fminimumf16 (function)
macro_rules! Depcrate_math_fminimum_fmaximumfminimumf16 {
() => {
// Module: crate::math::fminimum_fmaximum
// Provides: {"fminimumf16"}
// Dependencies: {}
# [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimumf16 (x : f16 , y : f16) -> f16 { super :: generic :: fminimum (x , y) }
};
}
