// Generated macro for impl_484 (impl)
macro_rules! Depcrate_matchers_matches_regex_matcherimpl_484 {
() => {
// Module: crate::matchers::matches_regex_matcher
// Provides: {"impl_484"}
// Dependencies: {}
impl < PatternT , ActualT > Matcher < ActualT > for MatchesRegexMatcher < PatternT > where PatternT : Deref < Target = str > , ActualT : AsRef < str > + Debug + Copy , { fn matches (& self , actual : ActualT) -> MatcherResult { self . regex . is_match (actual . as_ref ()) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("matches the regular expression {:#?}" , self . pattern . deref ()) . into () } MatcherResult :: NoMatch => { format ! ("doesn't match the regular expression {:#?}" , self . pattern . deref ()) . into () } } } }
};
}
