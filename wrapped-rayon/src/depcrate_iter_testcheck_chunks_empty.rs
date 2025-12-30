// Generated macro for check_chunks_empty (function)
macro_rules! Depcrate_iter_testcheck_chunks_empty {
() => {
// Module: crate::iter::test
// Provides: {"check_chunks_empty"}
// Dependencies: {}
# [test] fn check_chunks_empty () { let v : Vec < i32 > = vec ! [] ; let expected : Vec < Vec < i32 > > = vec ! [] ; assert_eq ! (expected , v . into_par_iter () . chunks (2) . collect ::< Vec < Vec < i32 >>> ()) ; }
};
}
