// Generated macro for StrMatcher (struct)
macro_rules! Depcrate_matchers_str_matcherStrMatcher {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"StrMatcher"}
// Dependencies: {}
# [doc = " A matcher which matches equality or containment of a string-like value in a"] # [doc = " configurable way."] # [doc = ""] # [doc = " The following matcher methods instantiate this:"] # [doc = ""] # [doc = "  * [`eq`][crate::matchers::eq_matcher::eq],"] # [doc = "  * [`contains_substring`],"] # [doc = "  * [`starts_with`],"] # [doc = "  * [`ends_with`]."] # [derive (MatcherBase)] pub struct StrMatcher < ExpectedT > { expected : ExpectedT , configuration : Configuration , }
};
}
