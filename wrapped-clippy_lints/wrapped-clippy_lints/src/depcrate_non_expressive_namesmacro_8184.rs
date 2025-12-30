// Generated macro for macro_8184 (macro)
macro_rules! Depcrate_non_expressive_namesmacro_8184 {
() => {
// Module: crate::non_expressive_names
// Provides: {"macro_8184"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for too many variables whose name consists of a"] # [doc = " single character."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's hard to memorize what a variable means without a"] # [doc = " descriptive name."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let (a, b, c, d, e, f, g) = (...);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MANY_SINGLE_CHAR_NAMES , pedantic , "too many single character bindings" }
};
}
