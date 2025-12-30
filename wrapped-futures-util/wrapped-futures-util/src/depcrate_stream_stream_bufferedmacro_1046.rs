// Generated macro for macro_1046 (macro)
macro_rules! Depcrate_stream_stream_bufferedmacro_1046 {
() => {
// Module: crate::stream::stream::buffered
// Provides: {"macro_1046"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`buffered`](super::StreamExt::buffered) method."] # [must_use = "streams do nothing unless polled"] pub struct Buffered < St > where St : Stream , St :: Item : Future , { # [pin] stream : Fuse < St >, in_progress_queue : FuturesOrdered < St :: Item >, max : Option < NonZeroUsize >, } }
};
}
