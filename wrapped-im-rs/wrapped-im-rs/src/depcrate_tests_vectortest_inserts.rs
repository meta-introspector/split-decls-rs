// Generated macro for test_inserts (function)
macro_rules! Depcrate_tests_vectortest_inserts {
() => {
// Module: crate::tests::vector
// Provides: {"test_inserts"}
// Dependencies: {}
# [test] fn test_inserts () { const N : usize = 2000 ; let mut v = Vector :: new () ; for i in 0 .. N { v . insert (v . len () / 2 , i) ; } let mut rv : Vec < usize > = Vec :: new () ; rv . extend ((0 .. N) . skip (1) . step_by (2)) ; rv . extend ((0 .. N) . step_by (2) . rev ()) ; assert_eq ! (rv . iter () . cloned () . collect ::< Vector < _ >> () , v) ; }
};
}
