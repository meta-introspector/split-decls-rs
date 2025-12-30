// Generated macro for macro_4991 (macro)
macro_rules! Depcrate_matchesmacro_4991 {
() => {
// Module: crate::matches
// Provides: {"macro_4991"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `match` which could be implemented using `filter`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using the `filter` method is clearer and more concise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " match Some(0) {"] # [doc = "     Some(x) => if x % 2 == 0 {"] # [doc = "                     Some(x)"] # [doc = "                } else {"] # [doc = "                     None"] # [doc = "                 },"] # [doc = "     None => None,"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " Some(0).filter(|&x| x % 2 == 0);"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub MANUAL_FILTER , complexity , "reimplementation of `filter`" }
};
}
