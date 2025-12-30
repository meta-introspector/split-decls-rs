// Generated macro for fmaximum_numf16 (function)
macro_rules! Depcrate_math_fminimum_fmaximum_numfmaximum_numf16 {
() => {
// Module: crate::math::fminimum_fmaximum_num
// Provides: {"fmaximum_numf16"}
// Dependencies: {}
# [doc = " Return the greater of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum_numf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmaximum_num (x , y) }
};
}
