// Generated macro for macro_1326 (macro)
macro_rules! Depcrate_defaultmacro_1326 {
() => {
// Module: crate::default
// Provides: {"macro_1326"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for literal calls to `Default::default()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's easier for the reader if the name of the type is used, rather than the"] # [doc = " generic `Default`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let s: String = Default::default();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let s = String::default();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DEFAULT_TRAIT_ACCESS , pedantic , "checks for literal calls to `Default::default()`" }
};
}
