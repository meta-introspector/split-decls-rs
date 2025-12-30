// Generated macro for get_leap_month_from_new_year (function)
macro_rules! Depcrate_chinese_basedget_leap_month_from_new_year {
() => {
// Module: crate::chinese_based
// Provides: {"get_leap_month_from_new_year"}
// Dependencies: {}
# [doc = " Given that `new_year` is the first day of a leap year, find which month in the year is a leap month."] # [doc = ""] # [doc = " Since the first month in which there are no major solar terms is a leap month, this function"] # [doc = " cycles through months until it finds the leap month, then returns the number of that month. This"] # [doc = " function assumes the date passed in is in a leap year and tests to ensure this is the case in debug"] # [doc = " mode by asserting that no more than thirteen months are analyzed."] # [doc = ""] # [doc = " Calls to `no_major_solar_term` have been inlined for increased efficiency."] # [doc = ""] # [doc = " Conceptually similar to code from _Calendrical Calculations_ by Reingold & Dershowitz"] # [doc = " Lisp reference code: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5443-L5450>"] pub fn get_leap_month_from_new_year < C : ChineseBased > (new_year : RataDie) -> u8 { let mut cur = new_year ; let mut result = 1 ; let mut solar_term = major_solar_term_from_fixed :: < C > (cur) ; loop { let next = new_moon_on_or_after :: < C > ((cur + 1) . as_moment ()) ; let next_solar_term = major_solar_term_from_fixed :: < C > (next) ; if result >= MAX_ITERS_FOR_MONTHS_OF_YEAR || solar_term == next_solar_term { break ; } cur = next ; solar_term = next_solar_term ; result += 1 ; } debug_assert ! (result < MAX_ITERS_FOR_MONTHS_OF_YEAR , "The given year was not a leap year and an unexpected number of iterations occurred searching for a leap month.") ; result }
};
}
