// Generated macro for macro_9254 (macro)
macro_rules! Depcrate_regexmacro_9254 {
() => {
// Module: crate::regex
// Provides: {"macro_9254"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks [regex](https://crates.io/crates/regex) creation"] # [doc = " (with `Regex::new`, `RegexBuilder::new`, or `RegexSet::new`) for correct"] # [doc = " regex syntax."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This will lead to a runtime panic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " Regex::new(\"(\")"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " Regex::new(\"\\(\")"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INVALID_REGEX , correctness , "invalid regular expressions" }
};
}
