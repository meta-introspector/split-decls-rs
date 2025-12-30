// Generated macro for macro_5040 (macro)
macro_rules! Depcrate_matchesmacro_5040 {
() => {
// Module: crate::matches
// Provides: {"macro_5040"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `match` which could be implemented using `map`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using the `map` method is clearer and more concise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " match Some(0) {"] # [doc = "     Some(x) => Some(x + 1),"] # [doc = "     None => None,"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " Some(0).map(|x| x + 1);"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub MANUAL_MAP , style , "reimplementation of `map`" }
};
}
