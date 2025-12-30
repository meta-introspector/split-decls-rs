// Generated macro for test_into_iter_clone_empty_smallvec (function)
macro_rules! Depcrate_teststest_into_iter_clone_empty_smallvec {
() => {
// Module: crate::tests
// Provides: {"test_into_iter_clone_empty_smallvec"}
// Dependencies: {}
# [test] fn test_into_iter_clone_empty_smallvec () { let mut iter = SmallVec :: < u8 , 2 > :: new () . into_iter () ; let mut clone_iter = iter . clone () ; assert_eq ! (iter . next () , None) ; assert_eq ! (clone_iter . next () , None) ; }
};
}
