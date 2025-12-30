// Generated macro for impl_502 (impl)
macro_rules! Depcrate_matchers_none_matcherimpl_502 {
() => {
// Module: crate::matchers::none_matcher
// Provides: {"impl_502"}
// Dependencies: {}
impl < T : Debug + Copy > Matcher < Option < T > > for NoneMatcher { fn matches (& self , actual : Option < T >) -> MatcherResult { actual . is_none () . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => "is none" . into () , MatcherResult :: NoMatch => "is some(_)" . into () , } } }
};
}
