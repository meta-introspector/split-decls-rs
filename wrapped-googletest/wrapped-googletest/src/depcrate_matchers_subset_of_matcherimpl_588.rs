// Generated macro for impl_588 (impl)
macro_rules! Depcrate_matchers_subset_of_matcherimpl_588 {
() => {
// Module: crate::matchers::subset_of_matcher
// Provides: {"impl_588"}
// Dependencies: {}
impl < ElementT : Debug + PartialEq + Copy , ActualT : Debug + Copy , ExpectedT : Debug > Matcher < ActualT > for SubsetOfMatcher < ExpectedT > where ActualT : IntoIterator < Item = ElementT > , for < 'a > & 'a ExpectedT : IntoIterator < Item = & 'a ElementT > , { fn matches (& self , actual : ActualT) -> MatcherResult { for actual_item in actual { if self . expected_is_missing (& actual_item) { return MatcherResult :: NoMatch ; } } MatcherResult :: Match } fn explain_match (& self , actual : ActualT) -> Description { let unexpected_elements = actual . into_iter () . enumerate () . filter (| item | self . expected_is_missing (& item . 1)) . map (| (idx , actual_item) | format ! ("{actual_item:#?} at #{idx}")) . collect :: < Vec < _ > > () ; match unexpected_elements . len () { 0 => "which no element is unexpected" . into () , 1 => format ! ("whose element {} is unexpected" , & unexpected_elements [0]) . into () , _ => format ! ("whose elements {} are unexpected" , unexpected_elements . join (", ")) . into () , } } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is a subset of {:#?}" , self . superset) . into () , MatcherResult :: NoMatch => format ! ("isn't a subset of {:#?}" , self . superset) . into () , } } }
};
}
