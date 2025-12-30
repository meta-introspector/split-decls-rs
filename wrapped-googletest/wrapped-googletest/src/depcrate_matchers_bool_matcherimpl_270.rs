// Generated macro for impl_270 (impl)
macro_rules! Depcrate_matchers_bool_matcherimpl_270 {
() => {
// Module: crate::matchers::bool_matcher
// Provides: {"impl_270"}
// Dependencies: {}
impl BoolMatcher { fn matches (& self , actual : bool) -> MatcherResult { (actual == self . expected) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match (matcher_result , self . expected) { (MatcherResult :: Match , true) | (MatcherResult :: NoMatch , false) => "is true" . into () , (MatcherResult :: Match , false) | (MatcherResult :: NoMatch , true) => "is false" . into () , } } }
};
}
