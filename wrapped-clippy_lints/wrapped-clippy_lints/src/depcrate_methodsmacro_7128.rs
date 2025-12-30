// Generated macro for macro_7128 (macro)
macro_rules! Depcrate_methodsmacro_7128 {
() => {
// Module: crate::methods
// Provides: {"macro_7128"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.chars().next()` on a `str` to check"] # [doc = " if it starts with a given char."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.starts_with(_)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let name = \"foo\";"] # [doc = " if name.chars().next() == Some('_') {};"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let name = \"foo\";"] # [doc = " if name.starts_with('_') {};"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CHARS_NEXT_CMP , style , "using `.chars().next()` to check if a string starts with a char" }
};
}
