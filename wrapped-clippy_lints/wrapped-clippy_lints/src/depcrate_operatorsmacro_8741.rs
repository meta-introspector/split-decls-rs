// Generated macro for macro_8741 (macro)
macro_rules! Depcrate_operatorsmacro_8741 {
() => {
// Module: crate::operators
// Provides: {"macro_8741"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for ineffective double comparisons against constants."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Only one of the comparisons has any effect on the result, the programmer"] # [doc = " probably intended to flip one of the comparison operators, or compare a"] # [doc = " different value entirely."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let status_code = 200;"] # [doc = " if status_code <= 400 && status_code < 500 {}"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub REDUNDANT_COMPARISONS , correctness , "double comparisons where one of them can be removed" }
};
}
