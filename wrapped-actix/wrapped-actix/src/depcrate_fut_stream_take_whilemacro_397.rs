// Generated macro for macro_397 (macro)
macro_rules! Depcrate_fut_stream_take_whilemacro_397 {
() => {
// Module: crate::fut::stream::take_while
// Provides: {"macro_397"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`take_while`](super::ActorStreamExt::take_while) method."] # [must_use = "streams do nothing unless polled"] # [derive (Debug)] pub struct TakeWhile < S , I , F , Fut > { # [pin] stream : S , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < I >, done_taking : bool , } }
};
}
