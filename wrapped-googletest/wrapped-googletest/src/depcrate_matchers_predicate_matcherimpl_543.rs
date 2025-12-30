// Generated macro for impl_543 (impl)
macro_rules! Depcrate_matchers_predicate_matcherimpl_543 {
() => {
// Module: crate::matchers::predicate_matcher
// Provides: {"impl_543"}
// Dependencies: {}
impl < T : Debug + Copy , P , D1 : PredicateDescription , D2 : PredicateDescription > Matcher < T > for PredicateMatcher < P , D1 , D2 > where P : Fn (T) -> bool , { fn matches (& self , actual : T) -> MatcherResult { (self . predicate) (actual) . into () } fn describe (& self , result : MatcherResult) -> Description { match result { MatcherResult :: Match => self . positive_description . to_description () , MatcherResult :: NoMatch => self . negative_description . to_description () , } } }
};
}
