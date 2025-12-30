// Generated macro for macro_7147 (macro)
macro_rules! Depcrate_methodsmacro_7147 {
() => {
// Module: crate::methods
// Provides: {"macro_7147"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.chars().last()` or"] # [doc = " `_.chars().next_back()` on a `str` to check if it ends with a given char."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.ends_with(_)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let name = \"_\";"] # [doc = " name.chars().last() == Some('_') || name.chars().next_back() == Some('-');"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let name = \"_\";"] # [doc = " name.ends_with('_') || name.ends_with('-');"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CHARS_LAST_CMP , style , "using `.chars().last()` or `.chars().next_back()` to check if a string ends with a char" }
};
}
