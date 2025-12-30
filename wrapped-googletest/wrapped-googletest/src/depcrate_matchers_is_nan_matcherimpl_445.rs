// Generated macro for impl_445 (impl)
macro_rules! Depcrate_matchers_is_nan_matcherimpl_445 {
() => {
// Module: crate::matchers::is_nan_matcher
// Provides: {"impl_445"}
// Dependencies: {}
impl < T : Float + Debug + Copy > Matcher < T > for IsNanMatcher { fn matches (& self , actual : T) -> MatcherResult { actual . is_nan () . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { if matcher_result . into () { "is NaN" } else { "isn't NaN" } . into () } }
};
}
