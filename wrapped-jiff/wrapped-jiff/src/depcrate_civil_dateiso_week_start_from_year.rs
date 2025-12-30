// Generated macro for iso_week_start_from_year (function)
macro_rules! Depcrate_civil_dateiso_week_start_from_year {
() => {
// Module: crate::civil::date
// Provides: {"iso_week_start_from_year"}
// Dependencies: {}
# [doc = " Returns the Unix epoch day corresponding to the first day in the ISO 8601"] # [doc = " week year given."] # [doc = ""] # [doc = " Ref: http://howardhinnant.github.io/date_algorithms.html"] fn iso_week_start_from_year (year : t :: ISOYear) -> UnixEpochDay { let date_in_first_week = Date :: new_ranged (year . rinto () , C (1) . rinto () , C (4) . rinto ()) . expect ("Jan 4 is valid for all valid years") ; let diff_from_monday = date_in_first_week . weekday () . since_ranged (Weekday :: Monday) ; date_in_first_week . to_unix_epoch_day () - diff_from_monday }
};
}
