// Generated macro for MatchesRegexMatcher (struct)
macro_rules! Depcrate_matchers_matches_regex_matcherMatchesRegexMatcher {
() => {
// Module: crate::matchers::matches_regex_matcher
// Provides: {"MatchesRegexMatcher"}
// Dependencies: {}
# [doc = " A matcher matching a string-like type matching a given regular expression."] # [doc = ""] # [doc = " Intended only to be used from the function [`matches_regex`] only."] # [doc = " Should not be referenced by code outside this library."] # [derive (MatcherBase)] pub struct MatchesRegexMatcher < PatternT : Deref < Target = str > > { regex : Regex , pattern : PatternT , _adjusted_pattern : String , }
};
}
