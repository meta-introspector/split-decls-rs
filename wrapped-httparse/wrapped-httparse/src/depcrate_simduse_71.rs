// Generated macro for use_71 (pub_use)
macro_rules! Depcrate_simduse_71 {
() => {
// Module: crate::simd
// Provides: {"use_71"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , target_feature = "sse4.2" , not (target_feature = "avx2") , any (target_arch = "x86" , target_arch = "x86_64" ,) ,))] pub use self :: sse42_compile_time :: * ;
};
}
