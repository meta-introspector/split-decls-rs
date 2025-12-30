// Generated macro for saturate_day_in_month (function)
macro_rules! Depcrate_civil_datesaturate_day_in_month {
() => {
// Module: crate::civil::date
// Provides: {"saturate_day_in_month"}
// Dependencies: {}
# [doc = " Saturates the given day in the month."] # [doc = ""] # [doc = " That is, if the day exceeds the maximum number of days in the given year"] # [doc = " and month, then this returns the maximum. Otherwise, it returns the day"] # [doc = " given."] # [inline] fn saturate_day_in_month (year : Year , month : Month , day : Day) -> Day { day . min (days_in_month (year , month)) }
};
}
