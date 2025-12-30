// Generated macro for impl_30 (impl)
macro_rules! Depcrate_streamimpl_30 {
() => {
// Module: crate::stream
// Provides: {"impl_30"}
// Dependencies: {}
impl MatchFinder { # [doc = " Test if this match finder is supported in this build of liblzma."] # [inline] pub fn is_supported (& self) -> bool { let ret = unsafe { liblzma_sys :: lzma_mf_is_supported (* self as liblzma_sys :: lzma_match_finder) } ; ret != 0 } }
};
}
