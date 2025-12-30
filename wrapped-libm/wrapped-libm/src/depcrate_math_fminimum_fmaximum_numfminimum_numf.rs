// Generated macro for fminimum_numf (function)
macro_rules! Depcrate_math_fminimum_fmaximum_numfminimum_numf {
() => {
// Module: crate::math::fminimum_fmaximum_num
// Provides: {"fminimum_numf"}
// Dependencies: {}
# [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_numf (x : f32 , y : f32) -> f32 { super :: generic :: fminimum_num (x , y) }
};
}
