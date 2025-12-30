// Generated macro for check_find_is_present (function)
macro_rules! Depcrate_iter_testcheck_find_is_present {
() => {
// Module: crate::iter::test
// Provides: {"check_find_is_present"}
// Dependencies: {}
# [test] fn check_find_is_present () { let counter = AtomicUsize :: new (0) ; let value : Option < i32 > = (0_i32 .. 2048) . into_par_iter () . find_any (| & p | { counter . fetch_add (1 , Ordering :: SeqCst) ; (1024 .. 1096) . contains (& p) }) ; let q = value . unwrap () ; assert ! ((1024 .. 1096) . contains (& q)) ; assert ! (counter . load (Ordering :: SeqCst) < 2048) ; }
};
}
