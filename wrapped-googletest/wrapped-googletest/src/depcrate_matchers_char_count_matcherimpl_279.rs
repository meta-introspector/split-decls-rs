// Generated macro for impl_279 (impl)
macro_rules! Depcrate_matchers_char_count_matcherimpl_279 {
() => {
// Module: crate::matchers::char_count_matcher
// Provides: {"impl_279"}
// Dependencies: {}
impl < T : Debug + Copy + AsRef < str > , E : Matcher < usize > > Matcher < T > for CharLenMatcher < E > { fn matches (& self , actual : T) -> MatcherResult { self . expected . matches (actual . as_ref () . chars () . count ()) } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("has character count, which {}" , self . expected . describe (MatcherResult :: Match)) . into () , MatcherResult :: NoMatch => format ! ("has character count, which {}" , self . expected . describe (MatcherResult :: NoMatch)) . into () , } } fn explain_match (& self , actual : T) -> Description { let actual_size = actual . as_ref () . chars () . count () ; format ! ("which has character count {}, {}" , actual_size , self . expected . explain_match (actual_size)) . into () } }
};
}
