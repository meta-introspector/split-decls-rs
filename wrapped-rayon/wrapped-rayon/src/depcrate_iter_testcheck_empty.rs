// Generated macro for check_empty (function)
macro_rules! Depcrate_iter_testcheck_empty {
() => {
// Module: crate::iter::test
// Provides: {"check_empty"}
// Dependencies: {}
# [test] fn check_empty () { let mut v : Vec < i32 > = empty () . filter (| _ | unreachable ! ()) . collect () ; assert ! (v . is_empty ()) ; empty () . collect_into_vec (& mut v) ; assert ! (v . is_empty ()) ; let v : Vec < (i32 , i32) > = empty () . zip (1 .. 10) . collect () ; assert ! (v . is_empty ()) ; }
};
}
