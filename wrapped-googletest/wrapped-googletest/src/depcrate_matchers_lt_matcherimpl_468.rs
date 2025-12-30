// Generated macro for impl_468 (impl)
macro_rules! Depcrate_matchers_lt_matcherimpl_468 {
() => {
// Module: crate::matchers::lt_matcher
// Provides: {"impl_468"}
// Dependencies: {}
impl < ActualT : Debug + PartialOrd < ExpectedT > + Copy , ExpectedT : Debug > Matcher < ActualT > for LtMatcher < ExpectedT > { fn matches (& self , actual : ActualT) -> MatcherResult { (actual < self . expected) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is less than {:?}" , self . expected) . into () , MatcherResult :: NoMatch => { format ! ("is greater than or equal to {:?}" , self . expected) . into () } } } }
};
}
