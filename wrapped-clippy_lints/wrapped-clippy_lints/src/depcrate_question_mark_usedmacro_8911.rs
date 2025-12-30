// Generated macro for macro_8911 (macro)
macro_rules! Depcrate_question_mark_usedmacro_8911 {
() => {
// Module: crate::question_mark_used
// Provides: {"macro_8911"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions that use the `?` operator and rejects them."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Sometimes code wants to avoid the `?` operator because for instance a local"] # [doc = " block requires a macro to re-throw errors to attach additional information to the"] # [doc = " error."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let result = expr?;"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " utility_macro!(expr);"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub QUESTION_MARK_USED , restriction , "checks if the `?` operator is used" }
};
}
