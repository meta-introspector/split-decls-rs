// Generated macro for macro_862 (macro)
macro_rules! Depcrate_stream_stream_skip_whilemacro_862 {
() => {
// Module: crate::stream::stream::skip_while
// Provides: {"macro_862"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`skip_while`](super::StreamExt::skip_while) method."] # [must_use = "streams do nothing unless polled"] pub struct SkipWhile < St , Fut , F > where St : Stream { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Item >, done_skipping : bool , } }
};
}
