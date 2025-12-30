// Generated macro for use_92 (pub_use)
macro_rules! Depcrate_simduse_92 {
() => {
// Module: crate::simd
// Provides: {"use_92"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , target_arch = "aarch64" , target_feature = "neon" ,))] pub use self :: neon :: * ;
};
}
