// Generated macro for check_hash_set (function)
macro_rules! Depcrate_iter_testcheck_hash_set {
() => {
// Module: crate::iter::test
// Provides: {"check_hash_set"}
// Dependencies: {}
# [test] fn check_hash_set () { use std :: collections :: HashSet ; let a : HashSet < i32 > = (0 .. 10) . collect () ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; assert_eq ! (45 , a . into_par_iter () . sum ::< i32 > ()) ; }
};
}
