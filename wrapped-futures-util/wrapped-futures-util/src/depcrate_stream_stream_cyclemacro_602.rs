// Generated macro for macro_602 (macro)
macro_rules! Depcrate_stream_stream_cyclemacro_602 {
() => {
// Module: crate::stream::stream::cycle
// Provides: {"macro_602"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`cycle`](super::StreamExt::cycle) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Cycle < St > { orig : St , # [pin] stream : St , } }
};
}
