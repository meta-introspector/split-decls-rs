// Generated macro for macro_7025 (macro)
macro_rules! Depcrate_methodsmacro_7025 {
() => {
// Module: crate::methods
// Provides: {"macro_7025"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual implementations of `str::repeat`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These are both harder to read, as well as less performant."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: String = std::iter::repeat('x').take(10).collect();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x: String = \"x\".repeat(10);"] # [doc = " ```"] # [clippy :: version = "1.54.0"] pub MANUAL_STR_REPEAT , perf , "manual implementation of `str::repeat`" }
};
}
