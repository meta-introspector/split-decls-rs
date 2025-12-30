// Generated macro for macro_1383 (macro)
macro_rules! Depcrate_stream_try_stream_try_ready_chunksmacro_1383 {
() => {
// Module: crate::stream::try_stream::try_ready_chunks
// Provides: {"macro_1383"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_ready_chunks`](super::TryStreamExt::try_ready_chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryReadyChunks < St : TryStream > { # [pin] stream : Fuse < IntoStream < St >>, cap : usize , } }
};
}
