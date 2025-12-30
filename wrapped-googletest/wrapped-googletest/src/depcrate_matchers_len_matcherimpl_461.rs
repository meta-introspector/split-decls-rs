// Generated macro for impl_461 (impl)
macro_rules! Depcrate_matchers_len_matcherimpl_461 {
() => {
// Module: crate::matchers::len_matcher
// Provides: {"impl_461"}
// Dependencies: {}
impl < T : Debug + Copy , E : Matcher < usize > > Matcher < T > for LenMatcher < E > where T : IntoIterator , { fn matches (& self , actual : T) -> MatcherResult { self . expected . matches (count_elements (actual)) } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("has length, which {}" , self . expected . describe (MatcherResult :: Match)) . into () } MatcherResult :: NoMatch => { format ! ("has length, which {}" , self . expected . describe (MatcherResult :: NoMatch)) . into () } } } fn explain_match (& self , actual : T) -> Description { let actual_size = count_elements (actual) ; format ! ("which has length {}, {}" , actual_size , self . expected . explain_match (actual_size)) . into () } }
};
}
