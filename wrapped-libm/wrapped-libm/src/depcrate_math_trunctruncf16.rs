// Generated macro for truncf16 (function)
macro_rules! Depcrate_math_trunctruncf16 {
() => {
// Module: crate::math::trunc
// Provides: {"truncf16"}
// Dependencies: {}
# [doc = " Rounds the number toward 0 to the closest integral value (f16)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn truncf16 (x : f16) -> f16 { super :: generic :: trunc (x) }
};
}
