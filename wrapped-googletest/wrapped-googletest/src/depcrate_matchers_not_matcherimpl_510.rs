// Generated macro for impl_510 (impl)
macro_rules! Depcrate_matchers_not_matcherimpl_510 {
() => {
// Module: crate::matchers::not_matcher
// Provides: {"impl_510"}
// Dependencies: {}
impl < T : Debug + Copy , InnerMatcherT : Matcher < T > > Matcher < T > for NotMatcher < InnerMatcherT > { fn matches (& self , actual : T) -> MatcherResult { match self . inner . matches (actual) { MatcherResult :: Match => MatcherResult :: NoMatch , MatcherResult :: NoMatch => MatcherResult :: Match , } } fn explain_match (& self , actual : T) -> Description { self . inner . explain_match (actual) } fn describe (& self , matcher_result : MatcherResult) -> Description { self . inner . describe (if matcher_result . into () { MatcherResult :: NoMatch } else { MatcherResult :: Match }) } }
};
}
