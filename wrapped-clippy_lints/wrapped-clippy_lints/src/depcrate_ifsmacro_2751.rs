// Generated macro for macro_2751 (macro)
macro_rules! Depcrate_ifsmacro_2751 {
() => {
// Module: crate::ifs
// Provides: {"macro_2751"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `if/else` with the same body as the *then* part"] # [doc = " and the *else* part."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably a copy & paste error."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let foo = if … {"] # [doc = "     42"] # [doc = " } else {"] # [doc = "     42"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub IF_SAME_THEN_ELSE , style , "`if` with the same `then` and `else` blocks" }
};
}
