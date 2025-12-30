// Generated macro for macro_1465 (macro)
macro_rules! Depcrate_stream_try_stream_try_bufferedmacro_1465 {
() => {
// Module: crate::stream::try_stream::try_buffered
// Provides: {"macro_1465"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_buffered`](super::TryStreamExt::try_buffered) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryBuffered < St > where St : TryStream , St :: Ok : TryFuture , { # [pin] stream : Fuse < IntoStream < St >>, in_progress_queue : FuturesOrdered < IntoFuture < St :: Ok >>, max : Option < NonZeroUsize >, } }
};
}
