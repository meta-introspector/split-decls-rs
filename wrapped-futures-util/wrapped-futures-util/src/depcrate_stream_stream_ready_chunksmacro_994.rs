// Generated macro for macro_994 (macro)
macro_rules! Depcrate_stream_stream_ready_chunksmacro_994 {
() => {
// Module: crate::stream::stream::ready_chunks
// Provides: {"macro_994"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`ready_chunks`](super::StreamExt::ready_chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct ReadyChunks < St : Stream > { # [pin] stream : Fuse < St >, cap : usize , } }
};
}
