// Generated macro for macro_630 (macro)
macro_rules! Depcrate_stream_stream_filtermacro_630 {
() => {
// Module: crate::stream::stream::filter
// Provides: {"macro_630"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`filter`](super::StreamExt::filter) method."] # [must_use = "streams do nothing unless polled"] pub struct Filter < St , Fut , F > where St : Stream , { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Item >, } }
};
}
