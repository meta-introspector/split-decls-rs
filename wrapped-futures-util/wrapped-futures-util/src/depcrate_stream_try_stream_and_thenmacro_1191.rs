// Generated macro for macro_1191 (macro)
macro_rules! Depcrate_stream_try_stream_and_thenmacro_1191 {
() => {
// Module: crate::stream::try_stream::and_then
// Provides: {"macro_1191"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`and_then`](super::TryStreamExt::and_then) method."] # [must_use = "streams do nothing unless polled"] pub struct AndThen < St , Fut , F > { # [pin] stream : St , # [pin] future : Option < Fut >, f : F , } }
};
}
