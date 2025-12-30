// Generated macro for macro_8735 (macro)
macro_rules! Depcrate_operatorsmacro_8735 {
() => {
// Module: crate::operators
// Provides: {"macro_8735"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `a op= a op b` or `a op= b op a` patterns."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Most likely these are bugs where one meant to write `a"] # [doc = " op= b`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Clippy cannot know for sure if `a op= a op b` should have"] # [doc = " been `a = a op a op b` or `a = a op b`/`a op= b`. Therefore, it suggests both."] # [doc = " If `a op= a op b` is really the correct behavior it should be"] # [doc = " written as `a = a op a op b` as it's less confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut a = 5;"] # [doc = " let b = 2;"] # [doc = " // ..."] # [doc = " a += a + b;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MISREFACTORED_ASSIGN_OP , suspicious , "having a variable on both sides of an assign op" }
};
}
