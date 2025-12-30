// Generated macro for is_arithmetic_leap_year (function)
macro_rules! Depcrate_persianis_arithmetic_leap_year {
() => {
// Module: crate::persian
// Provides: {"is_arithmetic_leap_year"}
// Dependencies: {}
# [doc = " Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4789"] # [doc = " Not used, but kept for comparative purposes"] # [allow (dead_code)] fn is_arithmetic_leap_year (p_year : i32) -> bool { let mut p_year = p_year as i64 ; if 0 < p_year { p_year -= 474 ; } else { p_year -= 473 ; } ; let year = p_year . rem_euclid (2820) + 474 ; ((year + 38) * 31) . rem_euclid (128) < 31 }
};
}
