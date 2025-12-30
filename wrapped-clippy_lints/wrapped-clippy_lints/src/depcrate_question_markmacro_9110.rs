// Generated macro for macro_9110 (macro)
macro_rules! Depcrate_question_markmacro_9110 {
() => {
// Module: crate::question_mark
// Provides: {"macro_9110"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions that could be replaced by the `?` operator."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using the `?` operator is shorter and more idiomatic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " if option.is_none() {"] # [doc = "     return None;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " option?;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub QUESTION_MARK , style , "checks for expressions that could be replaced by the `?` operator" }
};
}
