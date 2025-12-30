// Generated macro for macro_614 (macro)
macro_rules! Depcrate_stream_stream_enumeratemacro_614 {
() => {
// Module: crate::stream::stream::enumerate
// Provides: {"macro_614"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`enumerate`](super::StreamExt::enumerate) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Enumerate < St > { # [pin] stream : St , count : usize , } }
};
}
