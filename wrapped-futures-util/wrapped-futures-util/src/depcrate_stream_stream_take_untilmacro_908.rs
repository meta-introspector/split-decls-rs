// Generated macro for macro_908 (macro)
macro_rules! Depcrate_stream_stream_take_untilmacro_908 {
() => {
// Module: crate::stream::stream::take_until
// Provides: {"macro_908"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`take_until`](super::StreamExt::take_until) method."] # [must_use = "streams do nothing unless polled"] pub struct TakeUntil < St : Stream , Fut : Future > { # [pin] stream : St , # [pin] fut : Option < Fut >, fut_result : Option < Fut :: Output >, free : bool , } }
};
}
