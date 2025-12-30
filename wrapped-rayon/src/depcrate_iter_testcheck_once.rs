// Generated macro for check_once (function)
macro_rules! Depcrate_iter_testcheck_once {
() => {
// Module: crate::iter::test
// Provides: {"check_once"}
// Dependencies: {}
# [test] fn check_once () { let mut v : Vec < i32 > = once (42) . filter (| _ | true) . collect () ; assert_eq ! (v , & [42]) ; once (42) . collect_into_vec (& mut v) ; assert_eq ! (v , & [42]) ; let v : Vec < (i32 , i32) > = once (42) . zip (1 .. 10) . collect () ; assert_eq ! (v , & [(42 , 1)]) ; }
};
}
