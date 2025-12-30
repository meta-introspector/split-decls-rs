// Generated macro for macro_661 (macro)
macro_rules! Depcrate_stream_stream_flattenmacro_661 {
() => {
// Module: crate::stream::stream::flatten
// Provides: {"macro_661"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`flatten`](super::StreamExt::flatten) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Flatten < St , U > { # [pin] stream : St , # [pin] next : Option < U >, } }
};
}
