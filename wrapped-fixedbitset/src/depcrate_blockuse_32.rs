// Generated macro for use_32 (pub_use)
macro_rules! Depcrate_blockuse_32 {
() => {
// Module: crate::block
// Provides: {"use_32"}
// Dependencies: {}
# [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "sse2" , not (target_feature = "avx") , not (target_feature = "avx2") ,))] pub use self :: sse2 :: * ;
};
}
