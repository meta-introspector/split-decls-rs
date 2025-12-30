// Generated macro for macro_10839 (macro)
macro_rules! Depcrate_unit_typesmacro_10839 {
() => {
// Module: crate::unit_types
// Provides: {"macro_10839"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for passing a unit value as an argument to a function without using a"] # [doc = " unit literal (`()`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is likely the result of an accidental semicolon."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " foo({"] # [doc = "     let a = bar();"] # [doc = "     baz(a);"] # [doc = " })"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNIT_ARG , complexity , "passing unit to a function" }
};
}
