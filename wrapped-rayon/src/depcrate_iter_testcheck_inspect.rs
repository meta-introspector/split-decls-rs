// Generated macro for check_inspect (function)
macro_rules! Depcrate_iter_testcheck_inspect {
() => {
// Module: crate::iter::test
// Provides: {"check_inspect"}
// Dependencies: {}
# [test] fn check_inspect () { use std :: sync :: atomic :: { AtomicUsize , Ordering } ; let a = AtomicUsize :: new (0) ; let b : usize = (0_usize .. 1024) . into_par_iter () . inspect (| & i | { a . fetch_add (i , Ordering :: Relaxed) ; }) . sum () ; assert_eq ! (a . load (Ordering :: Relaxed) , b) ; }
};
}
