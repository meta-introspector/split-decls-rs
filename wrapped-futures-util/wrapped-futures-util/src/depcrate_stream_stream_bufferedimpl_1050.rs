// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_stream_stream_bufferedimpl_1050 {
() => {
// Module: crate::stream::stream::buffered
// Provides: {"impl_1050"}
// Dependencies: {}
impl < St > FusedStream for Buffered < St > where St : Stream , St :: Item : Future , { fn is_terminated (& self) -> bool { self . stream . is_done () && self . in_progress_queue . is_terminated () } }
};
}
