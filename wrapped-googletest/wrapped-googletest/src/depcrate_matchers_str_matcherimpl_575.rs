// Generated macro for impl_575 (impl)
macro_rules! Depcrate_matchers_str_matcherimpl_575 {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"impl_575"}
// Dependencies: {}
impl < T > StrMatcher < T > { # [doc = " Returns a [`StrMatcher`] with a default configuration to match against"] # [doc = " the given expected value."] # [doc = ""] # [doc = " This default configuration is sensitive to whitespace and case."] fn with_default_config (expected : T) -> Self { Self { expected , configuration : Default :: default () } } }
};
}
