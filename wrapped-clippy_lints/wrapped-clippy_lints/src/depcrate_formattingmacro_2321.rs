// Generated macro for macro_2321 (macro)
macro_rules! Depcrate_formattingmacro_2321 {
() => {
// Module: crate::formatting
// Provides: {"macro_2321"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of the non-existent `=*`, `=!` and `=-`"] # [doc = " operators."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is either a typo of `*=`, `!=` or `-=` or"] # [doc = " confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " a =- 42; // confusing, should it be `a -= 42` or `a = -42`?"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SUSPICIOUS_ASSIGNMENT_FORMATTING , suspicious , "suspicious formatting of `*=`, `-=` or `!=`" }
};
}
