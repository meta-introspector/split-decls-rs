// Generated macro for macro_1262 (macro)
macro_rules! Depcrate_copiesmacro_1262 {
() => {
// Module: crate::copies
// Provides: {"macro_1262"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `if/else` with the same body as the *then* part"] # [doc = " and the *else* part."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably a copy & paste error."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let foo = if … {"] # [doc = "     42"] # [doc = " } else {"] # [doc = "     42"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub IF_SAME_THEN_ELSE , style , "`if` with the same `then` and `else` blocks" }
};
}
