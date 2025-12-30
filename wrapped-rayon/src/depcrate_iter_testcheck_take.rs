// Generated macro for check_take (function)
macro_rules! Depcrate_iter_testcheck_take {
() => {
// Module: crate::iter::test
// Provides: {"check_take"}
// Dependencies: {}
# [test] fn check_take () { let a : Vec < usize > = (0 .. 1024) . collect () ; let mut v1 = Vec :: new () ; a . par_iter () . take (16) . collect_into_vec (& mut v1) ; let v2 = a . iter () . take (16) . collect :: < Vec < _ > > () ; assert_eq ! (v1 , v2) ; let mut v1 = Vec :: new () ; a . par_iter () . take (2048) . collect_into_vec (& mut v1) ; let v2 = a . iter () . take (2048) . collect :: < Vec < _ > > () ; assert_eq ! (v1 , v2) ; let mut v1 = Vec :: new () ; a . par_iter () . take (0) . collect_into_vec (& mut v1) ; let v2 = a . iter () . take (0) . collect :: < Vec < _ > > () ; assert_eq ! (v1 , v2) ; }
};
}
