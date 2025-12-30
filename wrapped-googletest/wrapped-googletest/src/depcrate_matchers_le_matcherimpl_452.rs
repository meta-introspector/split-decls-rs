// Generated macro for impl_452 (impl)
macro_rules! Depcrate_matchers_le_matcherimpl_452 {
() => {
// Module: crate::matchers::le_matcher
// Provides: {"impl_452"}
// Dependencies: {}
impl < ActualT : Debug + PartialOrd < ExpectedT > + Copy , ExpectedT : Debug > Matcher < ActualT > for LeMatcher < ExpectedT > { fn matches (& self , actual : ActualT) -> MatcherResult { (actual <= self . expected) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is less than or equal to {:?}" , self . expected) . into () , MatcherResult :: NoMatch => format ! ("is greater than {:?}" , self . expected) . into () , } } }
};
}
