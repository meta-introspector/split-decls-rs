// Generated macro for fmax (function)
macro_rules! Depcrate_math_fmin_fmaxfmax {
() => {
// Module: crate::math::fmin_fmax
// Provides: {"fmax"}
// Dependencies: {}
# [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmax (x : f64 , y : f64) -> f64 { super :: generic :: fmax (x , y) }
};
}
