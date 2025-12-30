// Generated macro for macro_9256 (macro)
macro_rules! Depcrate_regexmacro_9256 {
() => {
// Module: crate::regex
// Provides: {"macro_9256"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for [regex](https://crates.io/crates/regex) compilation inside a loop with a literal."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Compiling a regex is a much more expensive operation than using one, and a compiled regex can be used multiple times."] # [doc = " This is documented as an antipattern [on the regex documentation](https://docs.rs/regex/latest/regex/#avoid-re-compiling-regexes-especially-in-a-loop)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " # let haystacks = [\"\"];"] # [doc = " # const MY_REGEX: &str = \"a.b\";"] # [doc = " for haystack in haystacks {"] # [doc = "     let regex = regex::Regex::new(MY_REGEX).unwrap();"] # [doc = "     if regex.is_match(haystack) {"] # [doc = "         // Perform operation"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " can be replaced with"] # [doc = " ```rust,ignore"] # [doc = " # let haystacks = [\"\"];"] # [doc = " # const MY_REGEX: &str = \"a.b\";"] # [doc = " let regex = regex::Regex::new(MY_REGEX).unwrap();"] # [doc = " for haystack in haystacks {"] # [doc = "     if regex.is_match(haystack) {"] # [doc = "         // Perform operation"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.84.0"] pub REGEX_CREATION_IN_LOOPS , perf , "regular expression compilation performed in a loop" }
};
}
