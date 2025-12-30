// Generated macro for days_in_provided_year (function)
macro_rules! Depcrate_chinese_baseddays_in_provided_year {
() => {
// Module: crate::chinese_based
// Provides: {"days_in_provided_year"}
// Dependencies: {}
# [doc = " Calculated the numbers of days in the given year"] pub fn days_in_provided_year < C : ChineseBased > (year : i32) -> u16 { let mid_year = fixed_mid_year_from_year :: < C > (year) ; let bounds = YearBounds :: compute :: < C > (mid_year) ; bounds . count_days () }
};
}
