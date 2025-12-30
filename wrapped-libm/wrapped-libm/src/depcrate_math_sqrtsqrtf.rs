// Generated macro for sqrtf (function)
macro_rules! Depcrate_math_sqrtsqrtf {
() => {
// Module: crate::math::sqrt
// Provides: {"sqrtf"}
// Dependencies: {}
# [doc = " The square root of `x` (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrtf (x : f32) -> f32 { select_implementation ! { name : sqrtf , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , all (target_arch = "wasm32" , intrinsics_enabled) , target_feature = "sse2") , args : x , } super :: generic :: sqrt (x) }
};
}
