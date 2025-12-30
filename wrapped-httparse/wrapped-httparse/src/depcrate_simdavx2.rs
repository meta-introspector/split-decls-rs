// Generated macro for avx2 (module)
macro_rules! Depcrate_simdavx2 {
() => {
// Module: crate::simd
// Provides: {"avx2"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , any (target_feature = "avx2" , all (feature = "std" , not (target_feature = "sse4.2") ,) ,) , any (target_arch = "x86" , target_arch = "x86_64" ,) ,))] mod avx2 ;
};
}
