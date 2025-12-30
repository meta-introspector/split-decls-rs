// Generated macro for macro_10553 (macro)
macro_rules! Depcrate_unnecessary_owned_empty_stringsmacro_10553 {
() => {
// Module: crate::unnecessary_owned_empty_strings
// Provides: {"macro_10553"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Detects cases of owned empty strings being passed as an argument to a function expecting `&str`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " This results in longer and less readable code"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " vec![\"1\", \"2\", \"3\"].join(&String::new());"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " vec![\"1\", \"2\", \"3\"].join(\"\");"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub UNNECESSARY_OWNED_EMPTY_STRINGS , style , "detects cases of references to owned empty strings being passed as an argument to a function expecting `&str`" }
};
}
