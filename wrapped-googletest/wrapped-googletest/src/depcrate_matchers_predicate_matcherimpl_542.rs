// Generated macro for impl_542 (impl)
macro_rules! Depcrate_matchers_predicate_matcherimpl_542 {
() => {
// Module: crate::matchers::predicate_matcher
// Provides: {"impl_542"}
// Dependencies: {}
impl < T : Debug + Copy , P > Matcher < T > for PredicateMatcher < P , NoDescription , NoDescription > where P : Fn (T) -> bool , { fn matches (& self , actual : T) -> MatcherResult { (self . predicate) (actual) . into () } fn describe (& self , result : MatcherResult) -> Description { match result { MatcherResult :: Match => "matches" . into () , MatcherResult :: NoMatch => "does not match" . into () , } } }
};
}
