// Generated macro for is_leap_year (function)
macro_rules! Depcrate_persianis_leap_year {
() => {
// Module: crate::persian
// Provides: {"is_leap_year"}
// Dependencies: {}
# [doc = " Calculated using the 33-year rule"] pub fn is_leap_year (p_year : i32) -> bool { if p_year >= MIN_NON_LEAP_CORRECTION && NON_LEAP_CORRECTION . binary_search (& p_year) . is_ok () { false } else if p_year > MIN_NON_LEAP_CORRECTION && NON_LEAP_CORRECTION . binary_search (& (p_year - 1)) . is_ok () { true } else { let p_year = p_year as i64 ; (25 * p_year + 11) . rem_euclid (33) < 8 } }
};
}
