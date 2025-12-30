// Generated macro for macro_463 (macro)
macro_rules! Depcrate_attrsmacro_463 {
() => {
// Module: crate::attrs
// Provides: {"macro_463"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for ignored tests without messages."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The reason for ignoring the test may not be obvious."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[test]"] # [doc = " #[ignore]"] # [doc = " fn test() {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[test]"] # [doc = " #[ignore = \"Some good reason\"]"] # [doc = " fn test() {}"] # [doc = " ```"] # [clippy :: version = "1.88.0"] pub IGNORE_WITHOUT_REASON , pedantic , "ignored tests without messages" }
};
}
