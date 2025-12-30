// Generated macro for RegexMatcher (struct)
macro_rules! Depcrate_matcherRegexMatcher {
() => {
// Module: crate::matcher
// Provides: {"RegexMatcher"}
// Dependencies: {}
# [doc = " An implementation of the `Matcher` trait using Rust's standard regex"] # [doc = " library."] # [derive (Clone , Debug)] pub struct RegexMatcher { # [doc = " The configuration specified by the caller."] config : Config , # [doc = " The regular expression compiled from the pattern provided by the"] # [doc = " caller."] regex : Regex , # [doc = " A regex that never reports false negatives but may report false"] # [doc = " positives that is believed to be capable of being matched more quickly"] # [doc = " than `regex`. Typically, this is a single literal or an alternation"] # [doc = " of literals."] fast_line_regex : Option < Regex > , # [doc = " A set of bytes that will never appear in a match."] non_matching_bytes : ByteSet , }
};
}
