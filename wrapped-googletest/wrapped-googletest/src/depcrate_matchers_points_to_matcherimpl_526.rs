// Generated macro for impl_526 (impl)
macro_rules! Depcrate_matchers_points_to_matcherimpl_526 {
() => {
// Module: crate::matchers::points_to_matcher
// Provides: {"impl_526"}
// Dependencies: {}
impl < 'a , ExpectedT , MatcherT > Matcher < & 'a ExpectedT > for PointsToMatcher < MatcherT > where ExpectedT : Debug + Copy , MatcherT : Matcher < ExpectedT > , { fn matches (& self , actual : & 'a ExpectedT) -> MatcherResult { self . expected . matches (* actual) } fn explain_match (& self , actual : & 'a ExpectedT) -> Description { self . expected . explain_match (* actual) } fn describe (& self , matcher_result : MatcherResult) -> Description { self . expected . describe (matcher_result) } }
};
}
