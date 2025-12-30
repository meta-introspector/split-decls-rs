// Generated macro for macro_847 (macro)
macro_rules! Depcrate_stream_stream_skipmacro_847 {
() => {
// Module: crate::stream::stream::skip
// Provides: {"macro_847"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`skip`](super::StreamExt::skip) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Skip < St > { # [pin] stream : St , remaining : usize , } }
};
}
