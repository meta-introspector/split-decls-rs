// Generated macro for impl_431 (impl)
macro_rules! Depcrate_matchers_is_infinite_matcherimpl_431 {
() => {
// Module: crate::matchers::is_infinite_matcher
// Provides: {"impl_431"}
// Dependencies: {}
impl < T : Float + Debug + Copy > Matcher < T > for IsInfiniteMatcher { fn matches (& self , actual : T) -> MatcherResult { actual . is_infinite () . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { if matcher_result . into () { "is Infinite" } else { "isn't Infinite" } . into () } }
};
}
