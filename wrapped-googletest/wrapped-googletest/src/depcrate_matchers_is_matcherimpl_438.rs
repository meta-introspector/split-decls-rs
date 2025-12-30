// Generated macro for impl_438 (impl)
macro_rules! Depcrate_matchers_is_matcherimpl_438 {
() => {
// Module: crate::matchers::is_matcher
// Provides: {"impl_438"}
// Dependencies: {}
impl < ActualT : Debug + Copy , InnerMatcherT : Matcher < ActualT > > Matcher < ActualT > for IsMatcher < '_ , InnerMatcherT > { fn matches (& self , actual : ActualT) -> MatcherResult { self . inner . matches (actual) } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is {} which {}" , self . description , self . inner . describe (MatcherResult :: Match)) . into () , MatcherResult :: NoMatch => format ! ("is not {} which {}" , self . description , self . inner . describe (MatcherResult :: Match)) . into () , } } fn explain_match (& self , actual : ActualT) -> Description { self . inner . explain_match (actual) } }
};
}
