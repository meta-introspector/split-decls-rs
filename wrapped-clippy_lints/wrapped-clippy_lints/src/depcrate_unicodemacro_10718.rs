// Generated macro for macro_10718 (macro)
macro_rules! Depcrate_unicodemacro_10718 {
() => {
// Module: crate::unicode
// Provides: {"macro_10718"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for invisible Unicode characters in the code."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Having an invisible character in the code makes for all"] # [doc = " sorts of April fools, but otherwise is very much frowned upon."] # [doc = ""] # [doc = " ### Example"] # [doc = " You don't see it, but there may be a zero-width space or soft hyphen"] # [doc = " some\u{ad}where in this text."] # [clippy :: version = "1.49.0"] pub INVISIBLE_CHARACTERS , correctness , "using an invisible character in a string literal, which is confusing" }
};
}
