// Generated macro for impl_263 (impl)
macro_rules! Depcrate_matchers_anything_matcherimpl_263 {
() => {
// Module: crate::matchers::anything_matcher
// Provides: {"impl_263"}
// Dependencies: {}
impl < T : Debug + Copy > Matcher < T > for Anything { fn matches (& self , _ : T) -> MatcherResult { MatcherResult :: Match } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => "is anything" . into () , MatcherResult :: NoMatch => "never matches" . into () , } } }
};
}
