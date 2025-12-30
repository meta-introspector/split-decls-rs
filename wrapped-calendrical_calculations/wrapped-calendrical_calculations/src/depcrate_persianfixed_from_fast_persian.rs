// Generated macro for fixed_from_fast_persian (function)
macro_rules! Depcrate_persianfixed_from_fast_persian {
() => {
// Module: crate::persian
// Provides: {"fixed_from_fast_persian"}
// Dependencies: {}
# [doc = " fixed_from_arithmetic_persian, modified to use the more correct 33-year rule"] pub fn fixed_from_fast_persian (year : i32 , month : u8 , day : u8) -> RataDie { let p_year = i64 :: from (year) ; let month = i64 :: from (month) ; let day = i64 :: from (day) ; let mut new_year = FIXED_PERSIAN_EPOCH . to_i64_date () - 1 + 365 * (p_year - 1) + (8 * p_year + 21) . div_euclid (33) ; if year > MIN_NON_LEAP_CORRECTION && NON_LEAP_CORRECTION . binary_search (& (year - 1)) . is_ok () { new_year -= 1 ; } RataDie :: new (new_year - 1 + if month <= 7 { 31 * (month - 1) } else { 30 * (month - 1) + 6 } + day ,) }
};
}
