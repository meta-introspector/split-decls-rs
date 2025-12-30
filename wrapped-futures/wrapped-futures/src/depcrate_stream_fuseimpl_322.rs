// Generated macro for impl_322 (impl)
macro_rules! Depcrate_stream_fuseimpl_322 {
() => {
// Module: crate::stream::fuse
// Provides: {"impl_322"}
// Dependencies: {}
impl < S > Fuse < S > { # [doc = " Returns whether the underlying stream has finished or not."] # [doc = ""] # [doc = " If this method returns `true`, then all future calls to poll are"] # [doc = " guaranteed to return `NotReady`. If this returns `false`, then the"] # [doc = " underlying stream is still in use."] pub fn is_done (& self) -> bool { self . stream . is_none () } }
};
}
