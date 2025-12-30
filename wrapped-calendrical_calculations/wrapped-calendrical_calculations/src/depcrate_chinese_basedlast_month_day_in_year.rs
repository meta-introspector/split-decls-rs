// Generated macro for last_month_day_in_year (function)
macro_rules! Depcrate_chinese_basedlast_month_day_in_year {
() => {
// Module: crate::chinese_based
// Provides: {"last_month_day_in_year"}
// Dependencies: {}
# [doc = " The last month and day in this year"] pub fn last_month_day_in_year < C : ChineseBased > (year : i32) -> (u8 , u8) { let mid_year = fixed_mid_year_from_year :: < C > (year) ; let year_bounds = YearBounds :: compute :: < C > (mid_year) ; let last_day = year_bounds . next_new_year - 1 ; let month = if year_bounds . is_leap () { 13 } else { 12 } ; let day = last_day - new_moon_before :: < C > (last_day . as_moment ()) + 1 ; (month , day as u8) }
};
}
