// Generated macro for macro_8964 (macro)
macro_rules! Depcrate_raw_stringsmacro_8964 {
() => {
// Module: crate::raw_strings
// Provides: {"macro_8964"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for raw string literals with an unnecessary amount of hashes around them."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's just unnecessary, and makes it look like there's more escaping needed than is actually"] # [doc = " necessary."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let r = r###\"Hello, \"world\"!\"###;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let r = r#\"Hello, \"world\"!\"#;"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub NEEDLESS_RAW_STRING_HASHES , pedantic , "suggests reducing the number of hashes around a raw string literal" }
};
}
