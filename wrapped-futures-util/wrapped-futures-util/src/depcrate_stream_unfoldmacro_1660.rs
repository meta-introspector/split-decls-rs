// Generated macro for macro_1660 (macro)
macro_rules! Depcrate_stream_unfoldmacro_1660 {
() => {
// Module: crate::stream::unfold
// Provides: {"macro_1660"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`unfold`] function."] # [must_use = "streams do nothing unless polled"] pub struct Unfold < T , F , Fut > { f : F , # [pin] state : UnfoldState < T , Fut >, } }
};
}
