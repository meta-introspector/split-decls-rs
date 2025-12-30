// Generated macro for macro_1028 (macro)
macro_rules! Depcrate_stream_stream_buffer_unorderedmacro_1028 {
() => {
// Module: crate::stream::stream::buffer_unordered
// Provides: {"macro_1028"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`buffer_unordered`](super::StreamExt::buffer_unordered)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct BufferUnordered < St > where St : Stream , { # [pin] stream : Fuse < St >, in_progress_queue : FuturesUnordered < St :: Item >, max : Option < NonZeroUsize >, } }
};
}
