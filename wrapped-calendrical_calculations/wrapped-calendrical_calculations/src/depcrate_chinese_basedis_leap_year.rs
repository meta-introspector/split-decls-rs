// Generated macro for is_leap_year (function)
macro_rules! Depcrate_chinese_basedis_leap_year {
() => {
// Module: crate::chinese_based
// Provides: {"is_leap_year"}
// Dependencies: {}
# [doc = " Whether this year is a leap year"] pub fn is_leap_year < C : ChineseBased > (year : i32) -> bool { let mid_year = fixed_mid_year_from_year :: < C > (year) ; YearBounds :: compute :: < C > (mid_year) . is_leap () }
};
}
