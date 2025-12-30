// Generated macro for macro_8763 (macro)
macro_rules! Depcrate_precedencemacro_8763 {
() => {
// Module: crate::precedence
// Provides: {"macro_8763"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for operations where precedence may be unclear and suggests to add parentheses."] # [doc = " It catches a mixed usage of arithmetic and bit shifting/combining operators without parentheses"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Not everyone knows the precedence of those operators by"] # [doc = " heart, so expressions like these may trip others trying to reason about the"] # [doc = " code."] # [doc = ""] # [doc = " ### Example"] # [doc = " `1 << 2 + 3` equals 32, while `(1 << 2) + 3` equals 7"] # [clippy :: version = "pre 1.29.0"] pub PRECEDENCE , complexity , "operations where precedence may be unclear" }
};
}
