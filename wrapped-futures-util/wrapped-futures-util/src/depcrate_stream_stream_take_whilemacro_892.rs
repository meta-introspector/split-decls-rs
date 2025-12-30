// Generated macro for macro_892 (macro)
macro_rules! Depcrate_stream_stream_take_whilemacro_892 {
() => {
// Module: crate::stream::stream::take_while
// Provides: {"macro_892"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`take_while`](super::StreamExt::take_while) method."] # [must_use = "streams do nothing unless polled"] pub struct TakeWhile < St : Stream , Fut , F > { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Item >, done_taking : bool , } }
};
}
