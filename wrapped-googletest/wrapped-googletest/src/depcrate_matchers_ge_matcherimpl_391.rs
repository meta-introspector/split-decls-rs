// Generated macro for impl_391 (impl)
macro_rules! Depcrate_matchers_ge_matcherimpl_391 {
() => {
// Module: crate::matchers::ge_matcher
// Provides: {"impl_391"}
// Dependencies: {}
impl < ActualT : Debug + PartialOrd < ExpectedT > + Copy , ExpectedT : Debug > Matcher < ActualT > for GeMatcher < ExpectedT > { fn matches (& self , actual : ActualT) -> MatcherResult { (actual >= self . expected) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("is greater than or equal to {:?}" , self . expected) . into () } MatcherResult :: NoMatch => format ! ("is less than {:?}" , self . expected) . into () , } } }
};
}
