// Generated macro for macro_1417 (macro)
macro_rules! Depcrate_stream_try_stream_try_skip_whilemacro_1417 {
() => {
// Module: crate::stream::try_stream::try_skip_while
// Provides: {"macro_1417"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_skip_while`](super::TryStreamExt::try_skip_while)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TrySkipWhile < St , Fut , F > where St : TryStream { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Ok >, done_skipping : bool , } }
};
}
