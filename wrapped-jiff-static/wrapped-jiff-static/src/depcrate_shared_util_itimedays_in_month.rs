// Generated macro for days_in_month (function)
macro_rules! Depcrate_shared_util_itimedays_in_month {
() => {
// Module: crate::shared::util::itime
// Provides: {"days_in_month"}
// Dependencies: {}
# [doc = " Return the number of days in the given month."] # [inline] pub (crate) const fn days_in_month (year : i16 , month : i8) -> i8 { if month == 2 { if is_leap_year (year) { 29 } else { 28 } } else { 30 | (month ^ month >> 3) } }
};
}
