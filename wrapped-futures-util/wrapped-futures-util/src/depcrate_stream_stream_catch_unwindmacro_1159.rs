// Generated macro for macro_1159 (macro)
macro_rules! Depcrate_stream_stream_catch_unwindmacro_1159 {
() => {
// Module: crate::stream::stream::catch_unwind
// Provides: {"macro_1159"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`catch_unwind`](super::StreamExt::catch_unwind) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct CatchUnwind < St > { # [pin] stream : St , caught_unwind : bool , } }
};
}
