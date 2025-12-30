// Generated macro for macro_1450 (macro)
macro_rules! Depcrate_stream_try_stream_try_buffer_unorderedmacro_1450 {
() => {
// Module: crate::stream::try_stream::try_buffer_unordered
// Provides: {"macro_1450"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the"] # [doc = " [`try_buffer_unordered`](super::TryStreamExt::try_buffer_unordered) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryBufferUnordered < St > where St : TryStream { # [pin] stream : Fuse < IntoStream < St >>, in_progress_queue : FuturesUnordered < IntoFuture < St :: Ok >>, max : Option < NonZeroUsize >, } }
};
}
