// Generated macro for impl_380 (impl)
macro_rules! Depcrate_matchers_err_matcherimpl_380 {
() => {
// Module: crate::matchers::err_matcher
// Provides: {"impl_380"}
// Dependencies: {}
impl < 'a , T : Debug , E : Debug , InnerMatcherT : Matcher < & 'a E > > Matcher < & 'a std :: result :: Result < T , E > > for ErrMatcher < InnerMatcherT > { fn matches (& self , actual : & 'a std :: result :: Result < T , E >) -> MatcherResult { actual . as_ref () . err () . map (| v | self . inner . matches (v)) . unwrap_or (MatcherResult :: NoMatch) } fn explain_match (& self , actual : & 'a std :: result :: Result < T , E >) -> Description { match actual { Err (e) => { Description :: new () . text ("which is an error") . nested (self . inner . explain_match (e)) } Ok (_) => "which is a success" . into () , } } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("is an error which {}" , self . inner . describe (MatcherResult :: Match)) . into () } MatcherResult :: NoMatch => format ! ("is a success or is an error containing a value which {}" , self . inner . describe (MatcherResult :: NoMatch)) . into () , } } }
};
}
