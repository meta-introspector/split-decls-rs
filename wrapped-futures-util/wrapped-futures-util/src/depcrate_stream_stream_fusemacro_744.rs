// Generated macro for macro_744 (macro)
macro_rules! Depcrate_stream_stream_fusemacro_744 {
() => {
// Module: crate::stream::stream::fuse
// Provides: {"macro_744"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`fuse`](super::StreamExt::fuse) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Fuse < St > { # [pin] stream : St , done : bool , } }
};
}
