// Generated macro for macro_8524 (macro)
macro_rules! Depcrate_operatorsmacro_8524 {
() => {
// Module: crate::operators
// Provides: {"macro_8524"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for float arithmetic."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " For some embedded systems or kernel development, it"] # [doc = " can be useful to rule out floating-point numbers."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let a = 0.0;"] # [doc = " a + 1.0;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub FLOAT_ARITHMETIC , restriction , "any floating-point arithmetic statement" }
};
}
