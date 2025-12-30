// Generated macro for impl_423 (impl)
macro_rules! Depcrate_matchers_is_finite_matcherimpl_423 {
() => {
// Module: crate::matchers::is_finite_matcher
// Provides: {"impl_423"}
// Dependencies: {}
impl < T : Float + Debug + Copy > Matcher < T > for IsFiniteMatcher { fn matches (& self , actual : T) -> MatcherResult { actual . is_finite () . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { if matcher_result . into () { "is Finite" } else { "isn't Finite" } . into () } }
};
}
