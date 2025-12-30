// Generated macro for macro_8739 (macro)
macro_rules! Depcrate_operatorsmacro_8739 {
() => {
// Module: crate::operators
// Provides: {"macro_8739"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for double comparisons that could be simplified to a single expression."] # [doc = ""] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " # let y = 2;"] # [doc = " if x == y || x < y {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " # let y = 2;"] # [doc = " if x <= y {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DOUBLE_COMPARISONS , complexity , "unnecessary double comparisons that can be simplified" }
};
}
