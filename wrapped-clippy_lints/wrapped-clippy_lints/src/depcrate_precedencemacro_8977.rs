// Generated macro for macro_8977 (macro)
macro_rules! Depcrate_precedencemacro_8977 {
() => {
// Module: crate::precedence
// Provides: {"macro_8977"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for bit shifting operations combined with bit masking/combining operators"] # [doc = " and suggest using parentheses."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Not everyone knows the precedence of those operators by"] # [doc = " heart, so expressions like these may trip others trying to reason about the"] # [doc = " code."] # [doc = ""] # [doc = " ### Example"] # [doc = " `0x2345 & 0xF000 >> 12` equals 5, while `(0x2345 & 0xF000) >> 12` equals 2"] # [clippy :: version = "1.86.0"] pub PRECEDENCE_BITS , restriction , "operations mixing bit shifting with bit combining/masking" }
};
}
