// Generated macro for fixed_mid_year_from_year (function)
macro_rules! Depcrate_chinese_basedfixed_mid_year_from_year {
() => {
// Module: crate::chinese_based
// Provides: {"fixed_mid_year_from_year"}
// Dependencies: {}
# [doc = " Get a RataDie in the middle of a year."] # [doc = ""] # [doc = " This is not necessarily meant for direct use in"] # [doc = " calculations; rather, it is useful for getting a RataDie guaranteed to be in a given year"] # [doc = " as input for other calculations like calculating the leap month in a year."] # [doc = ""] # [doc = " Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz"] # [doc = " Lisp reference code: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5469-L5475>"] pub fn fixed_mid_year_from_year < C : ChineseBased > (elapsed_years : i32) -> RataDie { let cycle = (elapsed_years - 1) . div_euclid (60) + 1 ; let year = (elapsed_years - 1) . rem_euclid (60) + 1 ; C :: EPOCH + ((((cycle - 1) * 60 + year - 1) as f64 + 0.5) * MEAN_TROPICAL_YEAR) as i64 }
};
}
