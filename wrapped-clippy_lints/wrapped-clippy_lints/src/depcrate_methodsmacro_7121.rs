// Generated macro for macro_7121 (macro)
macro_rules! Depcrate_methodsmacro_7121 {
() => {
// Module: crate::methods
// Provides: {"macro_7121"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.skip_while(condition).next()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.find(!condition)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let vec = vec![1];"] # [doc = " vec.iter().skip_while(|x| **x == 0).next();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let vec = vec![1];"] # [doc = " vec.iter().find(|x| **x != 0);"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub SKIP_WHILE_NEXT , complexity , "using `skip_while(p).next()`, which is more succinctly expressed as `.find(!p)`" }
};
}
