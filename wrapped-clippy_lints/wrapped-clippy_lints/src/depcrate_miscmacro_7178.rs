// Generated macro for macro_7178 (macro)
macro_rules! Depcrate_miscmacro_7178 {
() => {
// Module: crate::misc
// Provides: {"macro_7178"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of short circuit boolean conditions as"] # [doc = " a"] # [doc = " statement."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using a short circuit boolean condition as a statement"] # [doc = " may hide the fact that the second part is executed or not depending on the"] # [doc = " outcome of the first part."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " f() && g(); // We should write `if f() { g(); }`."] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SHORT_CIRCUIT_STATEMENT , complexity , "using a short circuit boolean condition as a statement" }
};
}
