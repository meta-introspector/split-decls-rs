// Generated macro for macro_924 (macro)
macro_rules! Depcrate_stream_stream_thenmacro_924 {
() => {
// Module: crate::stream::stream::then
// Provides: {"macro_924"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`then`](super::StreamExt::then) method."] # [must_use = "streams do nothing unless polled"] pub struct Then < St , Fut , F > { # [pin] stream : St , # [pin] future : Option < Fut >, f : F , } }
};
}
