// Generated macro for is_leap_year (function)
macro_rules! Depcrate_offset_local_tz_info_ruleis_leap_year {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"is_leap_year"}
// Dependencies: {}
# [doc = " Check if a year is a leap year"] pub (crate) const fn is_leap_year (year : i32) -> bool { year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) }
};
}
