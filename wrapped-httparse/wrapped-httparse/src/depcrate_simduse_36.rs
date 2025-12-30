// Generated macro for use_36 (pub_use)
macro_rules! Depcrate_simduse_36 {
() => {
// Module: crate::simd
// Provides: {"use_36"}
// Dependencies: {}
# [cfg (any (httparse_disable_simd , miri , not (any (target_arch = "x86" , target_arch = "x86_64" , all (target_arch = "aarch64" , target_feature = "neon" ,))) , all (not (feature = "std") , not (any (target_feature = "sse4.2" , target_feature = "avx2" ,)) , any (target_arch = "x86" , target_arch = "x86_64" ,) ,)))] pub use self :: swar :: * ;
};
}
