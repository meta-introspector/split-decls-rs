// Generated macro for impl_296 (impl)
macro_rules! Depcrate_matchers_container_eq_matcherimpl_296 {
() => {
// Module: crate::matchers::container_eq_matcher
// Provides: {"impl_296"}
// Dependencies: {}
impl < ActualElementT , ActualContainerT , ExpectedElementT , ExpectedContainerT > Matcher < ActualContainerT > for ContainerEqMatcher < ExpectedContainerT > where ActualElementT : for < 'a > PartialEq < & 'a ExpectedElementT > + Debug + Copy , ActualContainerT : for < 'a > PartialEq < & 'a ExpectedContainerT > + Debug + Copy , ExpectedElementT : Debug , ExpectedContainerT : Debug , ActualContainerT : IntoIterator < Item = ActualElementT > , for < 'a > & 'a ExpectedContainerT : IntoIterator < Item = & 'a ExpectedElementT > , { fn matches (& self , actual : ActualContainerT) -> MatcherResult { (actual == & self . expected) . into () } fn explain_match (& self , actual : ActualContainerT) -> Description { build_explanation (self . get_missing_items (actual) , self . get_unexpected_items (actual)) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is equal to {:?}" , self . expected) . into () , MatcherResult :: NoMatch => format ! ("isn't equal to {:?}" , self . expected) . into () , } } }
};
}
