// Generated macro for fixed_from_arithmetic_persian (function)
macro_rules! Depcrate_persianfixed_from_arithmetic_persian {
() => {
// Module: crate::persian
// Provides: {"fixed_from_arithmetic_persian"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4803>"] # [doc = " Not used, but kept for comparative purposes"] pub fn fixed_from_arithmetic_persian (year : i32 , month : u8 , day : u8) -> RataDie { let p_year = i64 :: from (year) ; let month = i64 :: from (month) ; let day = i64 :: from (day) ; let y = if p_year > 0 { p_year - 474 } else { p_year - 473 } ; let year = y . rem_euclid (2820) + 474 ; RataDie :: new (FIXED_PERSIAN_EPOCH . to_i64_date () - 1 + 1029983 * y . div_euclid (2820) + 365 * (year - 1) + (31 * year - 5) . div_euclid (128) + if month <= 7 { 31 * (month - 1) } else { 30 * (month - 1) + 6 } + day ,) }
};
}
