// Generated macro for macro_8299 (macro)
macro_rules! Depcrate_nonstandard_macro_bracesmacro_8299 {
() => {
// Module: crate::nonstandard_macro_braces
// Provides: {"macro_8299"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks that common macros are used with consistent bracing."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Having non-conventional braces on well-stablished macros can be confusing"] # [doc = " when debugging, and they bring incosistencies with the rest of the ecosystem."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " vec!{1, 2, 3};"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " vec![1, 2, 3];"] # [doc = " ```"] # [clippy :: version = "1.55.0"] pub NONSTANDARD_MACRO_BRACES , nursery , "check consistent use of braces in macro" }
};
}
