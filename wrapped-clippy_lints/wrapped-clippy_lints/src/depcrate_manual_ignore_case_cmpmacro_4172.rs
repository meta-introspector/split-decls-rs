// Generated macro for macro_4172 (macro)
macro_rules! Depcrate_manual_ignore_case_cmpmacro_4172 {
() => {
// Module: crate::manual_ignore_case_cmp
// Provides: {"macro_4172"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual case-insensitive ASCII comparison."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `eq_ignore_ascii_case` method is faster because it does not allocate"] # [doc = " memory for the new strings, and it is more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn compare(a: &str, b: &str) -> bool {"] # [doc = "     a.to_ascii_lowercase() == b.to_ascii_lowercase() || a.to_ascii_lowercase() == \"abc\""] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn compare(a: &str, b: &str) -> bool {"] # [doc = "     a.eq_ignore_ascii_case(b) || a.eq_ignore_ascii_case(\"abc\")"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.84.0"] pub MANUAL_IGNORE_CASE_CMP , perf , "manual case-insensitive ASCII comparison" }
};
}
