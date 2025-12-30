// Generated macro for macro_1433 (macro)
macro_rules! Depcrate_stream_try_stream_try_take_whilemacro_1433 {
() => {
// Module: crate::stream::try_stream::try_take_while
// Provides: {"macro_1433"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_take_while`](super::TryStreamExt::try_take_while)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TryTakeWhile < St , Fut , F > where St : TryStream , { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Ok >, done_taking : bool , } }
};
}
