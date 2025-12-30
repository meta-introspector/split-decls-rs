// Generated macro for macro_9489 (macro)
macro_rules! Depcrate_regexmacro_9489 {
() => {
// Module: crate::regex
// Provides: {"macro_9489"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for trivial [regex](https://crates.io/crates/regex)"] # [doc = " creation (with `Regex::new`, `RegexBuilder::new`, or `RegexSet::new`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Matching the regex can likely be replaced by `==` or"] # [doc = " `str::starts_with`, `str::ends_with` or `std::contains` or other `str`"] # [doc = " methods."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the same regex is going to be applied to multiple"] # [doc = " inputs, the precomputations done by `Regex` construction can give"] # [doc = " significantly better performance than any of the `str`-based methods."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " Regex::new(\"^foobar\")"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " str::starts_with(\"foobar\")"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TRIVIAL_REGEX , nursery , "trivial regular expressions" }
};
}
