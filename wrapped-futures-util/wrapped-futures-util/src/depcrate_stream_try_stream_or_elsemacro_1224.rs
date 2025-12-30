// Generated macro for macro_1224 (macro)
macro_rules! Depcrate_stream_try_stream_or_elsemacro_1224 {
() => {
// Module: crate::stream::try_stream::or_else
// Provides: {"macro_1224"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`or_else`](super::TryStreamExt::or_else) method."] # [must_use = "streams do nothing unless polled"] pub struct OrElse < St , Fut , F > { # [pin] stream : St , # [pin] future : Option < Fut >, f : F , } }
};
}
