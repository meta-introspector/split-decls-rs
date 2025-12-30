// Generated macro for trunc (function)
macro_rules! Depcrate_math_trunctrunc {
() => {
// Module: crate::math::trunc
// Provides: {"trunc"}
// Dependencies: {}
# [doc = " Rounds the number toward 0 to the closest integral value (f64)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn trunc (x : f64) -> f64 { select_implementation ! { name : trunc , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: trunc (x) }
};
}
