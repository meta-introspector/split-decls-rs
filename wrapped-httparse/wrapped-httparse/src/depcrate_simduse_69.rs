// Generated macro for use_69 (pub_use)
macro_rules! Depcrate_simduse_69 {
() => {
// Module: crate::simd
// Provides: {"use_69"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , feature = "std" , not (any (target_feature = "sse4.2" , target_feature = "avx2" ,)) , any (target_arch = "x86" , target_arch = "x86_64" ,) ,))] pub use self :: runtime :: * ;
};
}
