// Generated macro for impl_572 (impl)
macro_rules! Depcrate_matchers_str_matcherimpl_572 {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"impl_572"}
// Dependencies: {}
impl < ExpectedT , ActualT > Matcher < ActualT > for StrMatcher < ExpectedT > where ExpectedT : Deref < Target = str > + Debug , ActualT : AsRef < str > + Debug + Copy , { fn matches (& self , actual : ActualT) -> MatcherResult { self . configuration . do_strings_match (self . expected . deref () , actual . as_ref ()) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { self . configuration . describe (matcher_result , self . expected . deref ()) } fn explain_match (& self , actual : ActualT) -> Description { self . configuration . explain_match (self . expected . deref () , actual . as_ref ()) } }
};
}
