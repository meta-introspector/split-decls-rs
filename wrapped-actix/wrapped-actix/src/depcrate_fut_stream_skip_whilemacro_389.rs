// Generated macro for macro_389 (macro)
macro_rules! Depcrate_fut_stream_skip_whilemacro_389 {
() => {
// Module: crate::fut::stream::skip_while
// Provides: {"macro_389"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`skip_while`](super::ActorStreamExt::skip_while) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct SkipWhile < S , I , F , Fut > { # [pin] stream : S , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < I >, done_skipping : bool , } }
};
}
