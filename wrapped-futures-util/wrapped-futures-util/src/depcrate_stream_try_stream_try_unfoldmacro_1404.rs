// Generated macro for macro_1404 (macro)
macro_rules! Depcrate_stream_try_stream_try_unfoldmacro_1404 {
() => {
// Module: crate::stream::try_stream::try_unfold
// Provides: {"macro_1404"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_unfold`] function."] # [must_use = "streams do nothing unless polled"] pub struct TryUnfold < T , F , Fut > { f : F , state : Option < T >, # [pin] fut : Option < Fut >, } }
};
}
