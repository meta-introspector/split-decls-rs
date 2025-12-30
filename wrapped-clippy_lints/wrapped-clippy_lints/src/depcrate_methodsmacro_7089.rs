// Generated macro for macro_7089 (macro)
macro_rules! Depcrate_methodsmacro_7089 {
() => {
// Module: crate::methods
// Provides: {"macro_7089"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if an iterator is used to check if a string is ascii."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `str` type already implements the `is_ascii` method."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " \"foo\".chars().all(|c| c.is_ascii());"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " \"foo\".is_ascii();"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub NEEDLESS_CHARACTER_ITERATION , suspicious , "is_ascii() called on a char iterator" }
};
}
