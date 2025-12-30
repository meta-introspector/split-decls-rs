// Generated macro for impl_272 (impl)
macro_rules! Depcrate_matchers_bool_matcherimpl_272 {
() => {
// Module: crate::matchers::bool_matcher
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a > Matcher < & 'a bool > for BoolMatcher { fn matches (& self , actual : & 'a bool) -> MatcherResult { self . matches (* actual) } fn describe (& self , matcher_result : MatcherResult) -> Description { self . describe (matcher_result) } }
};
}
