// Generated macro for impl_37 (impl)
macro_rules! Depcrate_streamimpl_37 {
() => {
// Module: crate::stream
// Provides: {"impl_37"}
// Dependencies: {}
impl Drop for Stream { # [inline] fn drop (& mut self) { unsafe { liblzma_sys :: lzma_end (& mut self . raw) ; } } }
};
}
