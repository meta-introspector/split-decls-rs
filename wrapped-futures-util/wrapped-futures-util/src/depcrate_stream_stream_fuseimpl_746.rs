// Generated macro for impl_746 (impl)
macro_rules! Depcrate_stream_stream_fuseimpl_746 {
() => {
// Module: crate::stream::stream::fuse
// Provides: {"impl_746"}
// Dependencies: {}
impl < S : Stream > FusedStream for Fuse < S > { fn is_terminated (& self) -> bool { self . done } }
};
}
