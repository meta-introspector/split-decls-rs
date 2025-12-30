// Generated macro for macro_1363 (macro)
macro_rules! Depcrate_stream_try_stream_try_chunksmacro_1363 {
() => {
// Module: crate::stream::try_stream::try_chunks
// Provides: {"macro_1363"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_chunks`](super::TryStreamExt::try_chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryChunks < St : TryStream > { # [pin] stream : Fuse < IntoStream < St >>, items : Vec < St :: Ok >, cap : usize , } }
};
}
