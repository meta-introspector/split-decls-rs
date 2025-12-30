// Generated macro for impl_596 (impl)
macro_rules! Depcrate_matchers_superset_of_matcherimpl_596 {
() => {
// Module: crate::matchers::superset_of_matcher
// Provides: {"impl_596"}
// Dependencies: {}
impl < ElementT : Debug + Copy + PartialEq , ActualT : Debug + Copy , ExpectedT : Debug > Matcher < ActualT > for SupersetOfMatcher < ExpectedT > where ActualT : IntoIterator < Item = ElementT > , for < 'a > & 'a ExpectedT : IntoIterator < Item = & 'a ElementT > , { fn matches (& self , actual : ActualT) -> MatcherResult { for expected_item in & self . subset { if actual_is_missing (actual , expected_item) { return MatcherResult :: NoMatch ; } } MatcherResult :: Match } fn explain_match (& self , actual : ActualT) -> Description { let missing_items : Vec < _ > = self . subset . into_iter () . filter (| expected_item | actual_is_missing (actual , * expected_item)) . map (| expected_item | format ! ("{expected_item:#?}")) . collect () ; match missing_items . len () { 0 => "whose no element is missing" . into () , 1 => format ! ("whose element {} is missing" , & missing_items [0]) . into () , _ => format ! ("whose elements {} are missing" , missing_items . join (", ")) . into () , } } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is a superset of {:#?}" , self . subset) . into () , MatcherResult :: NoMatch => format ! ("isn't a superset of {:#?}" , self . subset) . into () , } } }
};
}
