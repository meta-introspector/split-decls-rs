// Generated macro for macro_7134 (macro)
macro_rules! Depcrate_methodsmacro_7134 {
() => {
// Module: crate::methods
// Provides: {"macro_7134"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.to_string()` on an `&&T` where"] # [doc = " `T` implements `ToString` directly (like `&&str` or `&&String`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In versions of the compiler before Rust 1.82.0, this bypasses the specialized"] # [doc = " implementation of `ToString` and instead goes through the more expensive string"] # [doc = " formatting facilities."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // Generic implementation for `T: Display` is used (slow)"] # [doc = " [\"foo\", \"bar\"].iter().map(|s| s.to_string());"] # [doc = ""] # [doc = " // OK, the specialized impl is used"] # [doc = " [\"foo\", \"bar\"].iter().map(|&s| s.to_string());"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub INEFFICIENT_TO_STRING , pedantic , "using `to_string` on `&&T` where `T: ToString`" }
};
}
