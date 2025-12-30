// Generated macro for truncf128 (function)
macro_rules! Depcrate_math_trunctruncf128 {
() => {
// Module: crate::math::trunc
// Provides: {"truncf128"}
// Dependencies: {}
# [doc = " Rounds the number toward 0 to the closest integral value (f128)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn truncf128 (x : f128) -> f128 { super :: generic :: trunc (x) }
};
}
