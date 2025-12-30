// Generated macro for days_in_year (function)
macro_rules! Depcrate_shared_util_itimedays_in_year {
() => {
// Module: crate::shared::util::itime
// Provides: {"days_in_year"}
// Dependencies: {}
# [doc = " Return the number of days in the given year."] # [inline] pub (crate) const fn days_in_year (year : i16) -> i16 { if is_leap_year (year) { 366 } else { 365 } }
};
}
