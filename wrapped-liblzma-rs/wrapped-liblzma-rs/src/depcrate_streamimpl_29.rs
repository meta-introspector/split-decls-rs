// Generated macro for impl_29 (impl)
macro_rules! Depcrate_streamimpl_29 {
() => {
// Module: crate::stream
// Provides: {"impl_29"}
// Dependencies: {}
impl Check { # [doc = " Test if this check is supported in this build of liblzma."] # [inline] pub fn is_supported (& self) -> bool { let ret = unsafe { liblzma_sys :: lzma_check_is_supported (* self as liblzma_sys :: lzma_check) } ; ret != 0 } }
};
}
