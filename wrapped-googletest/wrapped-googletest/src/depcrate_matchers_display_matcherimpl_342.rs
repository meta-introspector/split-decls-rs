// Generated macro for impl_342 (impl)
macro_rules! Depcrate_matchers_display_matcherimpl_342 {
() => {
// Module: crate::matchers::display_matcher
// Provides: {"impl_342"}
// Dependencies: {}
impl < T : Debug + Display + Copy , InnerMatcher : for < 'a > Matcher < & 'a str > > Matcher < T > for DisplayMatcher < InnerMatcher > { fn matches (& self , actual : T) -> MatcherResult { self . inner . matches (& format ! ("{actual}")) } fn explain_match (& self , actual : T) -> Description { format ! ("which displays as {:?} {}" , actual . to_string () , self . inner . explain_match (& format ! ("{actual}"))) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("displays as a string which {}" , self . inner . describe (MatcherResult :: Match)) . into () } MatcherResult :: NoMatch => format ! ("doesn't display as a string which {}" , self . inner . describe (MatcherResult :: Match)) . into () , } } }
};
}
