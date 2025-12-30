// Generated macro for use_73 (pub_use)
macro_rules! Depcrate_simduse_73 {
() => {
// Module: crate::simd
// Provides: {"use_73"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , target_feature = "avx2" , any (target_arch = "x86" , target_arch = "x86_64" ,) ,))] pub use self :: avx2_compile_time :: * ;
};
}
