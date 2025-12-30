// Generated macro for major_solar_term_from_fixed (function)
macro_rules! Depcrate_chinese_basedmajor_solar_term_from_fixed {
() => {
// Module: crate::chinese_based
// Provides: {"major_solar_term_from_fixed"}
// Dependencies: {}
# [doc = " Get the current major solar term of a fixed date, output as an integer from 1..=12."] # [doc = ""] # [doc = " Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz."] # [doc = " Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5273-L5281"] pub (crate) fn major_solar_term_from_fixed < C : ChineseBased > (date : RataDie) -> u32 { let moment : Moment = date . as_moment () ; let universal = moment - C :: utc_offset (date) ; let solar_longitude = i64_to_i32 (Astronomical :: solar_longitude (Astronomical :: julian_centuries (universal)) as i64) ; debug_assert ! (solar_longitude . is_ok () , "Solar longitude should be in range of i32") ; let s = solar_longitude . unwrap_or_else (| e | e . saturate ()) ; let result_signed = (2 + s . div_euclid (30) - 1) . rem_euclid (12) + 1 ; debug_assert ! (result_signed >= 0) ; result_signed as u32 }
};
}
