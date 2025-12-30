// Generated macro for days_in_month (function)
macro_rules! Depcrate_civil_datedays_in_month {
() => {
// Module: crate::civil::date
// Provides: {"days_in_month"}
// Dependencies: {}
# [doc = " Returns the number of days in the given year and month."] # [doc = ""] # [doc = " This correctly returns `29` when the year is a leap year and the month is"] # [doc = " February."] # [inline] fn days_in_month (year : Year , month : Month) -> Day { let c = rangeint :: composite ! ((year , month) => { itime :: days_in_month (year , month) }) ; c . to_rint () }
};
}
