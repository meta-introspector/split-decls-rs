// Generated macro for NearMatcher (struct)
macro_rules! Depcrate_matchers_near_matcherNearMatcher {
() => {
// Module: crate::matchers::near_matcher
// Provides: {"NearMatcher"}
// Dependencies: {}
# [doc = " A matcher which matches floating-point numbers approximately equal to its"] # [doc = " expected value."] # [derive (MatcherBase)] pub struct NearMatcher < T : Debug > { expected : T , max_abs_error : T , nans_are_equal : bool , }
};
}
