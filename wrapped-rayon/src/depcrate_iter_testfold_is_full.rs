// Generated macro for fold_is_full (function)
macro_rules! Depcrate_iter_testfold_is_full {
() => {
// Module: crate::iter::test
// Provides: {"fold_is_full"}
// Dependencies: {}
# [test] fn fold_is_full () { let counter = AtomicUsize :: new (0) ; let a = (0_i32 .. 2048) . into_par_iter () . inspect (| _ | { counter . fetch_add (1 , Ordering :: SeqCst) ; }) . fold (| | 0 , | a , b | a + b) . find_any (| _ | true) ; assert ! (a . is_some ()) ; assert ! (counter . load (Ordering :: SeqCst) < 2048) ; }
};
}
