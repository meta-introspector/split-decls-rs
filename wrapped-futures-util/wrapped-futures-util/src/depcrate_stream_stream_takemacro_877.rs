// Generated macro for macro_877 (macro)
macro_rules! Depcrate_stream_stream_takemacro_877 {
() => {
// Module: crate::stream::stream::take
// Provides: {"macro_877"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`take`](super::StreamExt::take) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Take < St > { # [pin] stream : St , remaining : usize , } }
};
}
