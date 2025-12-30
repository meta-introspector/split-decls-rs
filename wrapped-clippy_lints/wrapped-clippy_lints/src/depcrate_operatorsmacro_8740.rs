// Generated macro for macro_8740 (macro)
macro_rules! Depcrate_operatorsmacro_8740 {
() => {
// Module: crate::operators
// Provides: {"macro_8740"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for double comparisons that can never succeed"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The whole expression can be replaced by `false`,"] # [doc = " which is probably not the programmer's intention"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let status_code = 200;"] # [doc = " if status_code <= 400 && status_code > 500 {}"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub IMPOSSIBLE_COMPARISONS , correctness , "double comparisons that will never evaluate to `true`" }
};
}
