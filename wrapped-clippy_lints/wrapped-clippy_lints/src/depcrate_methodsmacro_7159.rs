// Generated macro for macro_7159 (macro)
macro_rules! Depcrate_methodsmacro_7159 {
() => {
// Module: crate::methods
// Provides: {"macro_7159"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.as_ref().map(Deref::deref)` or its aliases (such as String::as_str)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.as_deref()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let opt = Some(\"\".to_string());"] # [doc = " opt.as_ref().map(String::as_str)"] # [doc = " # ;"] # [doc = " ```"] # [doc = " Can be written as"] # [doc = " ```no_run"] # [doc = " # let opt = Some(\"\".to_string());"] # [doc = " opt.as_deref()"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub OPTION_AS_REF_DEREF , complexity , "using `as_ref().map(Deref::deref)`, which is more succinctly expressed as `as_deref()`" }
};
}
