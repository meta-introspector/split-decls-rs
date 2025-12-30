// Generated macro for new_year_on_or_before_fixed_date (function)
macro_rules! Depcrate_chinese_basednew_year_on_or_before_fixed_date {
() => {
// Module: crate::chinese_based
// Provides: {"new_year_on_or_before_fixed_date"}
// Dependencies: {}
# [doc = " Get the fixed date of the nearest Lunar New Year on or before a given fixed date."] # [doc = " This function also returns the solstice following a given date for optimization (see #3743)."] # [doc = ""] # [doc = " To call this function you must precompute the value of the prior solstice, which"] # [doc = " is the result of winter_solstice_on_or_before"] # [doc = ""] # [doc = " Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz."] # [doc = " Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5396-L5405"] pub (crate) fn new_year_on_or_before_fixed_date < C : ChineseBased > (date : RataDie , prior_solstice : RataDie ,) -> (RataDie , RataDie) { let new_year = new_year_in_sui :: < C > (prior_solstice) ; if date >= new_year . 0 { new_year } else { let date_in_last_sui = date - 180 ; let prior_solstice = winter_solstice_on_or_before :: < C > (date_in_last_sui) ; new_year_in_sui :: < C > (prior_solstice) } }
};
}
