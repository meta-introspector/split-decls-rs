// Generated macro for impl_320 (impl)
macro_rules! Depcrate_matchers_contains_regex_matcherimpl_320 {
() => {
// Module: crate::matchers::contains_regex_matcher
// Provides: {"impl_320"}
// Dependencies: {}
impl < ActualT : AsRef < str > + Debug + Copy > Matcher < ActualT > for ContainsRegexMatcher { fn matches (& self , actual : ActualT) -> MatcherResult { self . regex . is_match (actual . as_ref ()) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("contains the regular expression {:#?}" , self . regex . as_str ()) . into () } MatcherResult :: NoMatch => { format ! ("doesn't contain the regular expression {:#?}" , self . regex . as_str ()) . into () } } } }
};
}
