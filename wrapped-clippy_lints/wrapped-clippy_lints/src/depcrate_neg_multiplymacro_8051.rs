// Generated macro for macro_8051 (macro)
macro_rules! Depcrate_neg_multiplymacro_8051 {
() => {
// Module: crate::neg_multiply
// Provides: {"macro_8051"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for multiplication by -1 as a form of negation."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's more readable to just negate."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let a = x * -1;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let a = -x;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEG_MULTIPLY , style , "multiplying integers by `-1`" }
};
}
