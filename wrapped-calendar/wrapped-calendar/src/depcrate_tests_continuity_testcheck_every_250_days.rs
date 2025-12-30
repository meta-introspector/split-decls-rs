// Generated macro for check_every_250_days (function)
macro_rules! Depcrate_tests_continuity_testcheck_every_250_days {
() => {
// Module: crate::tests::continuity_test
// Provides: {"check_every_250_days"}
// Dependencies: {}
fn check_every_250_days < A : AsCalendar > (mut date : Date < A > , iters : usize) { let duration = DateDuration :: for_days (250) ; let mut rata_die = date . to_rata_die () ; for _ in 0 .. iters { let next_date = date . try_added_with_options (duration , Default :: default ()) . unwrap () ; let next_iso = next_date . to_iso () ; let next_rata_die = next_iso . to_rata_die () ; assert_eq ! (next_rata_die , rata_die + 250 , "{next_date:?}") ; let next_date_roundtrip = next_iso . to_calendar (Ref (next_date . calendar ())) ; assert_eq ! (next_date , next_date_roundtrip , "{next_date:?}") ; date = next_date ; rata_die = next_rata_die ; } }
};
}
