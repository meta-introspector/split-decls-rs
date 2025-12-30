// Generated macro for ceilf (function)
macro_rules! Depcrate_math_ceilceilf {
() => {
// Module: crate::math::ceil
// Provides: {"ceilf"}
// Dependencies: {}
# [doc = " Ceil (f32)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceilf (x : f32) -> f32 { select_implementation ! { name : ceilf , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: ceil (x) }
};
}
