// Generated macro for fma (function)
macro_rules! Depcrate_math_fmafma {
() => {
// Module: crate::math::fma
// Provides: {"fma"}
// Dependencies: {}
# [doc = " Fused multiply add (f64)"] # [doc = ""] # [doc = " Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fma (x : f64 , y : f64 , z : f64) -> f64 { select_implementation ! { name : fma , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , target_feature = "sse2" ,) , args : x , y , z , } generic :: fma_round (x , y , z , Round :: Nearest) . val }
};
}
