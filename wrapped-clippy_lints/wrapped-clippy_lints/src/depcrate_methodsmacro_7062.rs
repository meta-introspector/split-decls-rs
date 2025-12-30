// Generated macro for macro_7062 (macro)
macro_rules! Depcrate_methodsmacro_7062 {
() => {
// Module: crate::methods
// Provides: {"macro_7062"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `.rev().next()` on a `DoubleEndedIterator`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.next_back()` is cleaner."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let foo = [0; 10];"] # [doc = " foo.iter().rev().next();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let foo = [0; 10];"] # [doc = " foo.iter().next_back();"] # [doc = " ```"] # [clippy :: version = "1.71.0"] pub MANUAL_NEXT_BACK , style , "manual reverse iteration of `DoubleEndedIterator`" }
};
}
