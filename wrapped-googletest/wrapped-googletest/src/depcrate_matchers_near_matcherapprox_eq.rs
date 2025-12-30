// Generated macro for approx_eq (function)
macro_rules! Depcrate_matchers_near_matcherapprox_eq {
() => {
// Module: crate::matchers::near_matcher
// Provides: {"approx_eq"}
// Dependencies: {}
# [doc = " Matches a value approximately equal to `expected`."] # [doc = ""] # [doc = " This automatically computes a tolerance from the magnitude of `expected` and"] # [doc = " matches any actual value within this tolerance of the expected value. The"] # [doc = " tolerance is chosen to account for the inaccuracies in most ordinary"] # [doc = " floating point calculations."] # [doc = ""] # [doc = " Otherwise this works analogously to [`near`]; see its documentation for"] # [doc = " further notes."] pub fn approx_eq < T : Debug + Float + FloatConst + Copy > (expected : T) -> NearMatcher < T > { let five_bits_of_mantissa = (T :: one () + T :: one ()) . powi (5) ; let abs_tolerance = five_bits_of_mantissa * T :: epsilon () ; let max_abs_error = T :: max (expected . abs () * abs_tolerance , abs_tolerance) ; NearMatcher { expected , max_abs_error , nans_are_equal : false } }
};
}
