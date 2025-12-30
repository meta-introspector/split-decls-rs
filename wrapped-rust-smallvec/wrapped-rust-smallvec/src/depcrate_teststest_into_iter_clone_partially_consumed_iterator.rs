// Generated macro for test_into_iter_clone_partially_consumed_iterator (function)
macro_rules! Depcrate_teststest_into_iter_clone_partially_consumed_iterator {
() => {
// Module: crate::tests
// Provides: {"test_into_iter_clone_partially_consumed_iterator"}
// Dependencies: {}
# [test] fn test_into_iter_clone_partially_consumed_iterator () { let mut iter = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) . into_iter () . skip (1) ; let mut clone_iter = iter . clone () ; while let Some (x) = iter . next () { assert_eq ! (x , clone_iter . next () . unwrap ()) ; } assert_eq ! (clone_iter . next () , None) ; }
};
}
