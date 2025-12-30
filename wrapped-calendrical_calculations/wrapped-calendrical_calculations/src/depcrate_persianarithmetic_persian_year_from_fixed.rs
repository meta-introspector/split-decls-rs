// Generated macro for arithmetic_persian_year_from_fixed (function)
macro_rules! Depcrate_persianarithmetic_persian_year_from_fixed {
() => {
// Module: crate::persian
// Provides: {"arithmetic_persian_year_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4829>"] # [doc = " Not used, but kept for comparative purposes"] fn arithmetic_persian_year_from_fixed (date : RataDie) -> i64 { let d0 = date - fixed_from_arithmetic_persian (475 , 1 , 1) ; let n2820 = d0 . div_euclid (1029983) ; let d1 = d0 . rem_euclid (1029983) ; let y2820 = if d1 == 1029982 { 2820 } else { (128 * d1 + 46878) . div_euclid (46751) } ; let year = 474 + n2820 * 2820 + y2820 ; if year > 0 { year } else { year - 1 } }
};
}
