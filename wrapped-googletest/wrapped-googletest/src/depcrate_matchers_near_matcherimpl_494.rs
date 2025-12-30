// Generated macro for impl_494 (impl)
macro_rules! Depcrate_matchers_near_matcherimpl_494 {
() => {
// Module: crate::matchers::near_matcher
// Provides: {"impl_494"}
// Dependencies: {}
impl < T : Borrow < F > + Debug + Copy , F : Debug + Float > Matcher < T > for NearMatcher < F > { fn matches (& self , actual : T) -> MatcherResult { if self . nans_are_equal && self . expected . is_nan () && actual . borrow () . is_nan () { return MatcherResult :: Match ; } let delta = * actual . borrow () - self . expected ; if delta >= - self . max_abs_error && delta <= self . max_abs_error { MatcherResult :: Match } else { MatcherResult :: NoMatch } } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => { format ! ("is within {:?} of {:?}" , self . max_abs_error , self . expected) . into () } MatcherResult :: NoMatch => { format ! ("isn't within {:?} of {:?}" , self . max_abs_error , self . expected) . into () } } } }
};
}
