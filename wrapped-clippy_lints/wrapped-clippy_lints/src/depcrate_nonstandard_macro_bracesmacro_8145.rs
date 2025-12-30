// Generated macro for macro_8145 (macro)
macro_rules! Depcrate_nonstandard_macro_bracesmacro_8145 {
() => {
// Module: crate::nonstandard_macro_braces
// Provides: {"macro_8145"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks that common macros are used with consistent bracing."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is mostly a consistency lint although using () or []"] # [doc = " doesn't give you a semicolon in item position, which can be unexpected."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " vec!{1, 2, 3};"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " vec![1, 2, 3];"] # [doc = " ```"] # [clippy :: version = "1.55.0"] pub NONSTANDARD_MACRO_BRACES , nursery , "check consistent use of braces in macro" }
};
}
