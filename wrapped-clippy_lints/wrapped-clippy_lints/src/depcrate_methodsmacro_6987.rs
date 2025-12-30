// Generated macro for macro_6987 (macro)
macro_rules! Depcrate_methodsmacro_6987 {
() => {
// Module: crate::methods
// Provides: {"macro_6987"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.to_string()` on an `&&T` where"] # [doc = " `T` implements `ToString` directly (like `&&str` or `&&String`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This bypasses the specialized implementation of"] # [doc = " `ToString` and instead goes through the more expensive string formatting"] # [doc = " facilities."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // Generic implementation for `T: Display` is used (slow)"] # [doc = " [\"foo\", \"bar\"].iter().map(|s| s.to_string());"] # [doc = ""] # [doc = " // OK, the specialized impl is used"] # [doc = " [\"foo\", \"bar\"].iter().map(|&s| s.to_string());"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub INEFFICIENT_TO_STRING , pedantic , "using `to_string` on `&&T` where `T: ToString`" }
};
}
