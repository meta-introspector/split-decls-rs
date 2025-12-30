// Generated macro for impl_360 (impl)
macro_rules! Depcrate_matchers_empty_matcherimpl_360 {
() => {
// Module: crate::matchers::empty_matcher
// Provides: {"impl_360"}
// Dependencies: {}
impl < T : Debug + Copy > Matcher < T > for EmptyMatcher where T : IntoIterator , { fn matches (& self , actual : T) -> MatcherResult { actual . into_iter () . next () . is_none () . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { if matcher_result . into () { "is empty" } else { "isn't empty" } . into () } }
};
}
