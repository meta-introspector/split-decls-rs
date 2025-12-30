// Generated macro for impl_1032 (impl)
macro_rules! Depcrate_stream_stream_buffer_unorderedimpl_1032 {
() => {
// Module: crate::stream::stream::buffer_unordered
// Provides: {"impl_1032"}
// Dependencies: {}
impl < St > FusedStream for BufferUnordered < St > where St : Stream , St :: Item : Future , { fn is_terminated (& self) -> bool { self . in_progress_queue . is_terminated () && self . stream . is_terminated () } }
};
}
