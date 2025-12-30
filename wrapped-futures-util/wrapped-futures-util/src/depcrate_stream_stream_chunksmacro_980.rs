// Generated macro for macro_980 (macro)
macro_rules! Depcrate_stream_stream_chunksmacro_980 {
() => {
// Module: crate::stream::stream::chunks
// Provides: {"macro_980"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`chunks`](super::StreamExt::chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Chunks < St : Stream > { # [pin] stream : Fuse < St >, items : Vec < St :: Item >, cap : usize , } }
};
}
