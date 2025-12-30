// Generated macro for impl_518 (impl)
macro_rules! Depcrate_matchers_ok_matcherimpl_518 {
() => {
// Module: crate::matchers::ok_matcher
// Provides: {"impl_518"}
// Dependencies: {}
impl < 'a , T : Debug , E : Debug , InnerMatcherT : Matcher < & 'a T > > Matcher < & 'a std :: result :: Result < T , E > > for OkMatcher < InnerMatcherT > { fn matches (& self , actual : & 'a std :: result :: Result < T , E >) -> MatcherResult { actual . as_ref () . map (| v | self . inner . matches (v)) . unwrap_or (MatcherResult :: NoMatch) } fn explain_match (& self , actual : & 'a std :: result :: Result < T , E >) -> Description { match actual { Ok (o) => { Description :: new () . text ("which is a success") . nested (self . inner . explain_match (o)) } Err (_) => "which is an error" . into () , } } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is a success containing a value, which {}" , self . inner . describe (MatcherResult :: Match)) . into () , MatcherResult :: NoMatch => format ! ("is an error or a success containing a value, which {}" , self . inner . describe (MatcherResult :: NoMatch)) . into () , } } }
};
}
