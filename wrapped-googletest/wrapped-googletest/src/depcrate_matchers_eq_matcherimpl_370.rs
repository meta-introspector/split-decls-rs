// Generated macro for impl_370 (impl)
macro_rules! Depcrate_matchers_eq_matcherimpl_370 {
() => {
// Module: crate::matchers::eq_matcher
// Provides: {"impl_370"}
// Dependencies: {}
impl < T : Debug , A : Debug + Copy + PartialEq < T > > Matcher < A > for EqMatcher < T > { fn matches (& self , actual : A) -> MatcherResult { (actual == self . expected) . into () } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("is equal to {:?}" , self . expected) . into () , MatcherResult :: NoMatch => format ! ("isn't equal to {:?}" , self . expected) . into () , } } fn explain_match (& self , actual : A) -> Description { let expected_debug = format ! ("{:#?}" , self . expected) ; let actual_debug = format ! ("{actual:#?}") ; let description = Matcher :: < A > :: describe (self , self . matches (actual)) ; let diff = if is_multiline_string_debug (& actual_debug) && is_multiline_string_debug (& expected_debug) { create_diff (& to_display_output (& actual_debug) . unwrap () , & to_display_output (& expected_debug) . unwrap () , edit_distance :: Mode :: Exact ,) } else { create_diff (& actual_debug , & expected_debug , edit_distance :: Mode :: Exact) } ; if diff . is_empty () { format ! ("which {description}") . into () } else { format ! ("which {description}\n\n{diff}") . into () } } }
};
}
