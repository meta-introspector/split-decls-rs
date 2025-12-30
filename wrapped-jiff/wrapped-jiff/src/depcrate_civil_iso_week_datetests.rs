// Generated macro for tests (module)
macro_rules! Depcrate_civil_iso_week_datetests {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"tests"}
// Dependencies: {}
# [cfg (not (miri))] # [cfg (test)] mod tests { use super :: * ; quickcheck :: quickcheck ! { fn prop_all_long_years_have_53rd_week (year : ISOYear) -> bool { ! is_long_year (year) || ISOWeekDate :: new (year . get () , 53 , Weekday :: Sunday) . is_ok () } fn prop_prev_day_is_less (wd : ISOWeekDate) -> quickcheck :: TestResult { use crate :: ToSpan ; if wd == ISOWeekDate :: MIN { return quickcheck :: TestResult :: discard () ; } let prev_date = wd . date () . checked_add (- 1 . days ()) . unwrap () ; quickcheck :: TestResult :: from_bool (prev_date . iso_week_date () < wd) } fn prop_next_day_is_greater (wd : ISOWeekDate) -> quickcheck :: TestResult { use crate :: ToSpan ; if wd == ISOWeekDate :: MAX { return quickcheck :: TestResult :: discard () ; } let next_date = wd . date () . checked_add (1 . days ()) . unwrap () ; quickcheck :: TestResult :: from_bool (wd < next_date . iso_week_date ()) } } }
};
}
