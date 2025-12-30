// Generated macro for rint (function)
macro_rules! Depcrate_math_rintrint {
() => {
// Module: crate::math::rint
// Provides: {"rint"}
// Dependencies: {}
# [doc = " Round `x` to the nearest integer, breaking ties toward even."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn rint (x : f64) -> f64 { select_implementation ! { name : rint , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , all (target_arch = "wasm32" , intrinsics_enabled) ,) , args : x , } super :: generic :: rint_round (x , Round :: Nearest) . val }
};
}
