// Generated macro for ceil (function)
macro_rules! Depcrate_math_ceilceil {
() => {
// Module: crate::math::ceil
// Provides: {"ceil"}
// Dependencies: {}
# [doc = " Ceil (f64)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceil (x : f64) -> f64 { select_implementation ! { name : ceil , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , use_arch_required : all (target_arch = "x86" , not (target_feature = "sse2")) , args : x , } super :: generic :: ceil (x) }
};
}
