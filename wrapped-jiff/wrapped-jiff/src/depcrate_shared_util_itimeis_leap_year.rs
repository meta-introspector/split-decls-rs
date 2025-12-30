// Generated macro for is_leap_year (function)
macro_rules! Depcrate_shared_util_itimeis_leap_year {
() => {
// Module: crate::shared::util::itime
// Provides: {"is_leap_year"}
// Dependencies: {}
# [doc = " Returns true if and only if the given year is a leap year."] # [doc = ""] # [doc = " A leap year is a year with 366 days. Typical years have 365 days."] # [inline] pub (crate) const fn is_leap_year (year : i16) -> bool { let d = if year % 25 != 0 { 4 } else { 16 } ; (year % d) == 0 }
};
}
