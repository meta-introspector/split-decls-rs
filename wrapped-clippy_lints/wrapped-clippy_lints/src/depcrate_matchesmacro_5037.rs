// Generated macro for macro_5037 (macro)
macro_rules! Depcrate_matchesmacro_5037 {
() => {
// Module: crate::matches
// Provides: {"macro_5037"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `match` expressions modifying the case of a string with non-compliant arms"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The arm is unreachable, which is likely a mistake"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let text = \"Foo\";"] # [doc = " match &*text.to_ascii_lowercase() {"] # [doc = "     \"foo\" => {},"] # [doc = "     \"Bar\" => {},"] # [doc = "     _ => {},"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let text = \"Foo\";"] # [doc = " match &*text.to_ascii_lowercase() {"] # [doc = "     \"foo\" => {},"] # [doc = "     \"bar\" => {},"] # [doc = "     _ => {},"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub MATCH_STR_CASE_MISMATCH , correctness , "creation of a case altering match expression with non-compliant arms" }
};
}
