// Generated macro for macro_7074 (macro)
macro_rules! Depcrate_methodsmacro_7074 {
() => {
// Module: crate::methods
// Provides: {"macro_7074"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `waker.clone().wake()`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Cloning the waker is not necessary, `wake_by_ref()` enables the same operation"] # [doc = " without extra cloning/dropping."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " waker.clone().wake();"] # [doc = " ```"] # [doc = " Should be written"] # [doc = " ```rust,ignore"] # [doc = " waker.wake_by_ref();"] # [doc = " ```"] # [clippy :: version = "1.75.0"] pub WAKER_CLONE_WAKE , perf , "cloning a `Waker` only to wake it" }
};
}
