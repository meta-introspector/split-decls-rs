// Generated macro for impl_398 (impl)
macro_rules! Depcrate_matchers_gt_matcherimpl_398 {
() => {
// Module: crate::matchers::gt_matcher
// Provides: {"impl_398"}
// Dependencies: {}
impl < ActualT : Debug + PartialOrd < ExpectedT > + Copy , ExpectedT : Debug > Matcher < ActualT > for GtMatcher < ExpectedT > { fn matches (& self , actual : ActualT) -> MatcherResult { (actual > self . expected) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is greater than {:?}" , self . expected) . into () , MatcherResult :: NoMatch => { format ! ("is less than or equal to {:?}" , self . expected) . into () } } } }
};
}
