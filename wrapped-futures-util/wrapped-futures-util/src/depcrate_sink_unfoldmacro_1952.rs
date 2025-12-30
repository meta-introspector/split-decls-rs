// Generated macro for macro_1952 (macro)
macro_rules! Depcrate_sink_unfoldmacro_1952 {
() => {
// Module: crate::sink::unfold
// Provides: {"macro_1952"}
// Dependencies: {}
pin_project ! { # [doc = " Sink for the [`unfold`] function."] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct Unfold < T , F , Fut > { function : F , # [pin] state : UnfoldState < T , Fut >, } }
};
}
