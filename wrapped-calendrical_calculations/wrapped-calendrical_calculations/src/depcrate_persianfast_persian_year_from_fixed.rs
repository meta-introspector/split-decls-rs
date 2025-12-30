// Generated macro for fast_persian_year_from_fixed (function)
macro_rules! Depcrate_persianfast_persian_year_from_fixed {
() => {
// Module: crate::persian
// Provides: {"fast_persian_year_from_fixed"}
// Dependencies: {}
# [doc = " arithmetic_persian_year_from_fixed modified for the 33-year rule"] fn fast_persian_year_from_fixed (date : RataDie) -> i64 { let days_since_epoch = date - FIXED_PERSIAN_EPOCH + 1 ; 1 + (33 * days_since_epoch + 3) . div_euclid (12053) }
};
}
