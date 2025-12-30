// Generated macro for impl_503 (impl)
macro_rules! Depcrate_matchers_none_matcherimpl_503 {
() => {
// Module: crate::matchers::none_matcher
// Provides: {"impl_503"}
// Dependencies: {}
impl < 'a , T : Debug > Matcher < & 'a Option < T > > for NoneMatcher { fn matches (& self , actual : & 'a Option < T >) -> MatcherResult { actual . is_none () . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => "is none" . into () , MatcherResult :: NoMatch => "is some(_)" . into () , } } }
};
}
