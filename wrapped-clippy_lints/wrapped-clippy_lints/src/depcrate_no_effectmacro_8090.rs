// Generated macro for macro_8090 (macro)
macro_rules! Depcrate_no_effectmacro_8090 {
() => {
// Module: crate::no_effect
// Provides: {"macro_8090"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expression statements that can be reduced to a"] # [doc = " sub-expression."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Expressions by themselves often have no side-effects."] # [doc = " Having such expressions reduces readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " compute_array()[0];"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNNECESSARY_OPERATION , complexity , "outer expressions with no effect" }
};
}
