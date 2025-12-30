// Generated macro for macro_405 (macro)
macro_rules! Depcrate_fut_stream_thenmacro_405 {
() => {
// Module: crate::fut::stream::then
// Provides: {"macro_405"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`then`](super::ActorStreamExt::then) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Then < S , F , Fut > { # [pin] stream : S , # [pin] future : Option < Fut >, f : F , } }
};
}
