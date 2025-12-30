// Generated macro for impl_559 (impl)
macro_rules! Depcrate_matchers_some_matcherimpl_559 {
() => {
// Module: crate::matchers::some_matcher
// Provides: {"impl_559"}
// Dependencies: {}
impl < T : Debug + Copy , InnerMatcherT : Matcher < T > > Matcher < Option < T > > for SomeMatcher < InnerMatcherT > { fn matches (& self , actual : Option < T >) -> MatcherResult { actual . map (| v | self . inner . matches (v)) . unwrap_or (MatcherResult :: NoMatch) } fn explain_match (& self , actual : Option < T >) -> Description { match (self . matches (actual) , actual) { (_ , Some (t)) => { Description :: new () . text ("which has a value") . nested (self . inner . explain_match (t)) } (_ , None) => "which is None" . into () , } } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("has a value which {}" , self . inner . describe (MatcherResult :: Match)) . into () } MatcherResult :: NoMatch => format ! ("is None or has a value which {}" , self . inner . describe (MatcherResult :: NoMatch)) . into () , } } }
};
}
