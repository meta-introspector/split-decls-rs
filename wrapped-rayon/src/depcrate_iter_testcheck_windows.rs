// Generated macro for check_windows (function)
macro_rules! Depcrate_iter_testcheck_windows {
() => {
// Module: crate::iter::test
// Provides: {"check_windows"}
// Dependencies: {}
# [test] fn check_windows () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let par : Vec < _ > = a . par_windows (2) . collect () ; let seq : Vec < _ > = a . windows (2) . collect () ; assert_eq ! (par , seq) ; let par : Vec < _ > = a . par_windows (100) . collect () ; let seq : Vec < _ > = a . windows (100) . collect () ; assert_eq ! (par , seq) ; let par : Vec < _ > = a . par_windows (1_000_000) . collect () ; let seq : Vec < _ > = a . windows (1_000_000) . collect () ; assert_eq ! (par , seq) ; let par : Vec < _ > = a . par_windows (2) . chain (a . par_windows (1_000_000)) . zip (a . par_windows (2)) . collect () ; let seq : Vec < _ > = a . windows (2) . chain (a . windows (1_000_000)) . zip (a . windows (2)) . collect () ; assert_eq ! (par , seq) ; }
};
}
