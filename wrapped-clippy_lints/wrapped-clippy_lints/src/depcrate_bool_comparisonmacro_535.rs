// Generated macro for macro_535 (macro)
macro_rules! Depcrate_bool_comparisonmacro_535 {
() => {
// Module: crate::bool_comparison
// Provides: {"macro_535"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions of the form `x == true`,"] # [doc = " `x != true` and order comparisons such as `x < true` (or vice versa) and"] # [doc = " suggest using the variable directly."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unnecessary code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " if x == true {}"] # [doc = " if y == false {}"] # [doc = " ```"] # [doc = " use `x` directly:"] # [doc = " ```rust,ignore"] # [doc = " if x {}"] # [doc = " if !y {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub BOOL_COMPARISON , complexity , "comparing a variable to a boolean, e.g., `if x == true` or `if x != true`" }
};
}
