// Generated macro for macro_7127 (macro)
macro_rules! Depcrate_methodsmacro_7127 {
() => {
// Module: crate::methods
// Provides: {"macro_7127"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for an iterator or string search (such as `find()`,"] # [doc = " `position()`, or `rposition()`) followed by a call to `is_some()` or `is_none()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as:"] # [doc = " * `_.any(_)`, or `_.contains(_)` for `is_some()`,"] # [doc = " * `!_.any(_)`, or `!_.contains(_)` for `is_none()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let vec = vec![1];"] # [doc = " vec.iter().find(|x| **x == 0).is_some();"] # [doc = ""] # [doc = " \"hello world\".find(\"world\").is_none();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let vec = vec![1];"] # [doc = " vec.iter().any(|x| *x == 0);"] # [doc = ""] # [doc = " !\"hello world\".contains(\"world\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SEARCH_IS_SOME , complexity , "using an iterator or string search followed by `is_some()` or `is_none()`, which is more succinctly expressed as a call to `any()` or `contains()` (with negation in case of `is_none()`)" }
};
}
