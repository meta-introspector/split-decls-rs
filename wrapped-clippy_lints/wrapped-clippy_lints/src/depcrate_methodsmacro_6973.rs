// Generated macro for macro_6973 (macro)
macro_rules! Depcrate_methodsmacro_6973 {
() => {
// Module: crate::methods
// Provides: {"macro_6973"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.filter(_).next()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.find(_)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let vec = vec![1];"] # [doc = " vec.iter().filter(|x| **x == 0).next();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let vec = vec![1];"] # [doc = " vec.iter().find(|x| **x == 0);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub FILTER_NEXT , complexity , "using `filter(p).next()`, which is more succinctly expressed as `.find(p)`" }
};
}
