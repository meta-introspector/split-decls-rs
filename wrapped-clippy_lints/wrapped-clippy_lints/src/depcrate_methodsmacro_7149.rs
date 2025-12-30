// Generated macro for macro_7149 (macro)
macro_rules! Depcrate_methodsmacro_7149 {
() => {
// Module: crate::methods
// Provides: {"macro_7149"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `fold` when a more succinct alternative exists."] # [doc = " Specifically, this checks for `fold`s which could be replaced by `any`, `all`,"] # [doc = " `sum` or `product`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " (0..3).fold(false, |acc, x| acc || x > 2);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " (0..3).any(|x| x > 2);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNNECESSARY_FOLD , style , "using `fold` when a more succinct alternative exists" }
};
}
