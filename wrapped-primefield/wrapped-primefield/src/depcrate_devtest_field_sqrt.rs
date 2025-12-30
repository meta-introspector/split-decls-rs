// Generated macro for test_field_sqrt (macro)
macro_rules! Depcrate_devtest_field_sqrt {
() => {
// Module: crate::dev
// Provides: {"test_field_sqrt"}
// Dependencies: {}
# [doc = " Implement field element square root tests."] # [macro_export] macro_rules ! test_field_sqrt { ($ fe : tt) => { # [test] fn sqrt () { for & n in & [1u64 , 4 , 9 , 16 , 25 , 36 , 49 , 64] { let fe = $ fe :: from (n) ; let sqrt = $ crate :: ff :: Field :: sqrt (& fe) . unwrap () ; assert_eq ! (sqrt . square () , fe) ; } } } ; }
};
}
