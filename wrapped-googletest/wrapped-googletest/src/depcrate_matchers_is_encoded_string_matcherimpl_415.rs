// Generated macro for impl_415 (impl)
macro_rules! Depcrate_matchers_is_encoded_string_matcherimpl_415 {
() => {
// Module: crate::matchers::is_encoded_string_matcher
// Provides: {"impl_415"}
// Dependencies: {}
impl < ActualT : AsRef < [u8] > + Debug + Copy , InnerMatcherT > Matcher < ActualT > for IsEncodedStringMatcher < InnerMatcherT > where InnerMatcherT : for < 'a > Matcher < & 'a str > , { fn matches (& self , actual : ActualT) -> MatcherResult { std :: str :: from_utf8 (actual . as_ref ()) . map (| s | self . inner . matches (s)) . unwrap_or (MatcherResult :: NoMatch) } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is a UTF-8 encoded string which {}" , self . inner . describe (MatcherResult :: Match)) . into () , MatcherResult :: NoMatch => format ! ("is not a UTF-8 encoded string which {}" , self . inner . describe (MatcherResult :: Match)) . into () , } } fn explain_match (& self , actual : ActualT) -> Description { match std :: str :: from_utf8 (actual . as_ref ()) { Ok (s) => { format ! ("which is a UTF-8 encoded string {}" , self . inner . explain_match (s)) . into () } Err (e) => format ! ("which is not a UTF-8 encoded string: {e}") . into () , } } }
};
}
