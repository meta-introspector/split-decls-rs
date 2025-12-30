// Generated macro for macro_539 (macro)
macro_rules! Depcrate_stream_stream_chainmacro_539 {
() => {
// Module: crate::stream::stream::chain
// Provides: {"macro_539"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`chain`](super::StreamExt::chain) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Chain < St1 , St2 > { # [pin] first : Option < St1 >, # [pin] second : St2 , } }
};
}
