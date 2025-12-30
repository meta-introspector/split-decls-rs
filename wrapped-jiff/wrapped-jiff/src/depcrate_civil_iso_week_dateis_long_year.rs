// Generated macro for is_long_year (function)
macro_rules! Depcrate_civil_iso_week_dateis_long_year {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"is_long_year"}
// Dependencies: {}
# [doc = " Returns true if the given ISO year is a \"long\" year or not."] # [doc = ""] # [doc = " A \"long\" year is a year with 53 weeks. Otherwise, it's a \"short\" year"] # [doc = " with 52 weeks."] fn is_long_year (year : ISOYear) -> bool { let last = Date :: new_ranged (year . rinto () , C (12) . rinto () , C (31) . rinto ()) . expect ("last day of year is always valid") ; let weekday = last . weekday () ; weekday == Weekday :: Thursday || (last . in_leap_year () && weekday == Weekday :: Friday) }
};
}
