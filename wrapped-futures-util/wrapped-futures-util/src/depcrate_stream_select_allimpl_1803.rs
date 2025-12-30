// Generated macro for impl_1803 (impl)
macro_rules! Depcrate_stream_select_allimpl_1803 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1803"}
// Dependencies: {}
impl < St : Stream + Unpin > FusedStream for SelectAll < St > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
};
}
