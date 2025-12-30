// Generated macro for ContainsMatcher (struct)
macro_rules! Depcrate_matchers_contains_matcherContainsMatcher {
() => {
// Module: crate::matchers::contains_matcher
// Provides: {"ContainsMatcher"}
// Dependencies: {}
# [doc = " A matcher which matches a container containing one or more elements a given"] # [doc = " inner [`Matcher`] matches."] # [derive (MatcherBase)] pub struct ContainsMatcher < InnerMatcherT > { inner : InnerMatcherT , count : Option < Box < dyn Matcher < usize > > > , }
};
}
