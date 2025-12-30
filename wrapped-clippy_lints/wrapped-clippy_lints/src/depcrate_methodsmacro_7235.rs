// Generated macro for macro_7235 (macro)
macro_rules! Depcrate_methodsmacro_7235 {
() => {
// Module: crate::methods
// Provides: {"macro_7235"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It identifies calls to `.is_empty()` on constant values."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " String literals and constant values are known at compile time. Checking if they"] # [doc = " are empty will always return the same value. This might not be the intention of"] # [doc = " the expression."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let value = \"\";"] # [doc = " if value.is_empty() {"] # [doc = "     println!(\"the string is empty\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " println!(\"the string is empty\");"] # [doc = " ```"] # [clippy :: version = "1.79.0"] pub CONST_IS_EMPTY , suspicious , "is_empty() called on strings known at compile time" }
};
}
