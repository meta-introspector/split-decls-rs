// Generated macro for sse42 (module)
macro_rules! Depcrate_simdsse42 {
() => {
// Module: crate::simd
// Provides: {"sse42"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , any (feature = "std" , target_feature = "sse4.2" ,) , not (target_feature = "avx2") , any (target_arch = "x86" , target_arch = "x86_64" ,) ,))] mod sse42 ;
};
}
