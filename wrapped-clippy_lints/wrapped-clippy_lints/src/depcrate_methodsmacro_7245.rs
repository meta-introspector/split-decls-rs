// Generated macro for macro_7245 (macro)
macro_rules! Depcrate_methodsmacro_7245 {
() => {
// Module: crate::methods
// Provides: {"macro_7245"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for `repeat().take()` that can be replaced with `repeat_n()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Using `repeat_n()` is more concise and clearer. Also, `repeat_n()` is sometimes faster than `repeat().take()` when the type of the element is non-trivial to clone because the original value can be reused for the last `.next()` call rather than always cloning."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::repeat(10).take(3);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::repeat_n(10, 3);"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub MANUAL_REPEAT_N , style , "detect `repeat().take()` that can be replaced with `repeat_n()`" }
};
}
