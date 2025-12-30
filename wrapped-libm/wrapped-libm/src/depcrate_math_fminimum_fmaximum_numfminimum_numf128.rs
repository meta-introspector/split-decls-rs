// Generated macro for fminimum_numf128 (function)
macro_rules! Depcrate_math_fminimum_fmaximum_numfminimum_numf128 {
() => {
// Module: crate::math::fminimum_fmaximum_num
// Provides: {"fminimum_numf128"}
// Dependencies: {}
# [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_numf128 (x : f128 , y : f128) -> f128 { super :: generic :: fminimum_num (x , y) }
};
}
