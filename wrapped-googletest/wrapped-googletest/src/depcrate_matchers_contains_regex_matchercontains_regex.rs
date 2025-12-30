// Generated macro for contains_regex (function)
macro_rules! Depcrate_matchers_contains_regex_matchercontains_regex {
() => {
// Module: crate::matchers::contains_regex_matcher
// Provides: {"contains_regex"}
// Dependencies: {}
# [doc = " Matches a string containing a substring which matches the given regular"] # [doc = " expression."] # [doc = ""] # [doc = " Both the actual value and the expected regular expression may be either a"] # [doc = " `String` or a string reference."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass_1() -> Result<()> {"] # [doc = " verify_that!(\"Some value\", contains_regex(\"S.*e\"))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail() -> Result<()> {"] # [doc = " verify_that!(\"Another value\", contains_regex(\"Some\"))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_pass_2() -> Result<()> {"] # [doc = " verify_that!(\"Some value\".to_string(), contains_regex(\"v.*e\"))?;   // Passes"] # [doc = " verify_that!(\"Some value\", contains_regex(\"v.*e\".to_string()))?;   // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass_1().unwrap();"] # [doc = " # should_fail().unwrap_err();"] # [doc = " # should_pass_2().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " Panics if the given `pattern` is not a syntactically valid regular"] # [doc = " expression."] # [track_caller] pub fn contains_regex < PatternT : Deref < Target = str > > (pattern : PatternT) -> ContainsRegexMatcher { ContainsRegexMatcher { regex : Regex :: new (pattern . deref ()) . unwrap () } }
};
}
