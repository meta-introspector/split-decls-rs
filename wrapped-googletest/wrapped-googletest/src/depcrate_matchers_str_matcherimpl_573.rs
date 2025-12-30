// Generated macro for impl_573 (impl)
macro_rules! Depcrate_matchers_str_matcherimpl_573 {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"impl_573"}
// Dependencies: {}
impl < ExpectedT , MatcherT : Into < StrMatcher < ExpectedT > > > StrMatcherConfigurator < ExpectedT > for MatcherT { fn ignoring_leading_whitespace (self) -> StrMatcher < ExpectedT > { let existing = self . into () ; StrMatcher { configuration : existing . configuration . ignoring_leading_whitespace () , .. existing } } fn ignoring_trailing_whitespace (self) -> StrMatcher < ExpectedT > { let existing = self . into () ; StrMatcher { configuration : existing . configuration . ignoring_trailing_whitespace () , .. existing } } fn ignoring_outer_whitespace (self) -> StrMatcher < ExpectedT > { let existing = self . into () ; StrMatcher { configuration : existing . configuration . ignoring_outer_whitespace () , .. existing } } fn ignoring_ascii_case (self) -> StrMatcher < ExpectedT > { let existing = self . into () ; StrMatcher { configuration : existing . configuration . ignoring_ascii_case () , .. existing } } fn ignoring_unicode_case (self) -> StrMatcher < ExpectedT > { let existing = self . into () ; StrMatcher { configuration : existing . configuration . ignoring_unicode_case () , .. existing } } fn times (self , times : impl Matcher < usize > + 'static) -> StrMatcher < ExpectedT > { let existing = self . into () ; if ! matches ! (existing . configuration . mode , MatchMode :: Contains) { panic ! ("The times() configurator is only meaningful with contains_substring().") ; } StrMatcher { configuration : existing . configuration . times (times) , .. existing } } }
};
}
