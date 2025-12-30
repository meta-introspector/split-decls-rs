// Generated macro for wrap_std_iter_into_fallible (function)
macro_rules! Depcrate_testwrap_std_iter_into_fallible {
() => {
// Module: crate::test
// Provides: {"wrap_std_iter_into_fallible"}
// Dependencies: {}
# [test] fn wrap_std_iter_into_fallible () { let it = IntoFallible :: from (vec ! [0 , 1 , 2 , 3] . into_iter ()) ; assert_eq ! (it . collect ::< Vec < _ >> () . unwrap () , vec ! [0 , 1 , 2 , 3]) ; }
};
}
