// Generated macro for sse2 (module)
macro_rules! Depcrate_blocksse2 {
() => {
// Module: crate::block
// Provides: {"sse2"}
// Dependencies: {}
# [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "sse2" , not (target_feature = "avx") , not (target_feature = "avx2") ,))] mod sse2 ;
};
}
