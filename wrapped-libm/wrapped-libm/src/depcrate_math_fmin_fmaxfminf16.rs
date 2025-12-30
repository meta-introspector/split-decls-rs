// Generated macro for fminf16 (function)
macro_rules! Depcrate_math_fmin_fmaxfminf16 {
() => {
// Module: crate::math::fmin_fmax
// Provides: {"fminf16"}
// Dependencies: {}
# [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmin (x , y) }
};
}
