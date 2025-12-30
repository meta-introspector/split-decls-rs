// Generated macro for check_continuity (function)
macro_rules! Depcrate_tests_continuity_testcheck_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"check_continuity"}
// Dependencies: {}
fn check_continuity < A : AsCalendar > (mut date : Date < A > , years_to_check : usize) { let duration = DateDuration :: for_days (1) ; let mut rata_die = date . to_rata_die () ; let mut weekday = date . day_of_week () ; let mut year = date . year () ; let mut is_in_leap_year = date . is_in_leap_year () ; for _ in 0 .. (366 * years_to_check) { let next_date = date . try_added_with_options (duration , Default :: default ()) . unwrap () ; let next_rata_die = next_date . to_iso () . to_rata_die () ; assert_eq ! (next_rata_die , rata_die + 1 , "{next_date:?}") ; let next_weekday = next_date . day_of_week () ; let next_year = next_date . year () ; let next_is_in_leap_year = next_date . is_in_leap_year () ; assert_eq ! ((next_weekday as usize) % 7 , (weekday as usize + 1) % 7 , "{next_date:?}") ; if year == next_year { assert_eq ! (is_in_leap_year , next_is_in_leap_year , "{next_date:?}") ; } date = next_date ; rata_die = next_rata_die ; weekday = next_weekday ; year = next_year ; is_in_leap_year = next_is_in_leap_year ; } }
};
}
