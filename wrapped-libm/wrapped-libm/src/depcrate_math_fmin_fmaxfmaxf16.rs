// Generated macro for fmaxf16 (function)
macro_rules! Depcrate_math_fmin_fmaxfmaxf16 {
() => {
// Module: crate::math::fmin_fmax
// Provides: {"fmaxf16"}
// Dependencies: {}
# [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaxf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmax (x , y) }
};
}
