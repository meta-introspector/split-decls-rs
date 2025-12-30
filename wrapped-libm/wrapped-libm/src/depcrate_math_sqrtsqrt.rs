// Generated macro for sqrt (function)
macro_rules! Depcrate_math_sqrtsqrt {
() => {
// Module: crate::math::sqrt
// Provides: {"sqrt"}
// Dependencies: {}
# [doc = " The square root of `x` (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrt (x : f64) -> f64 { select_implementation ! { name : sqrt , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , all (target_arch = "wasm32" , intrinsics_enabled) , target_feature = "sse2") , args : x , } super :: generic :: sqrt (x) }
};
}
