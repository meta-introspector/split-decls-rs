// Generated macro for fminf (function)
macro_rules! Depcrate_math_fmin_fmaxfminf {
() => {
// Module: crate::math::fmin_fmax
// Provides: {"fminf"}
// Dependencies: {}
# [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf (x : f32 , y : f32) -> f32 { super :: generic :: fmin (x , y) }
};
}
