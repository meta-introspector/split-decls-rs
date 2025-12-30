// Generated macro for test_into_vec (function)
macro_rules! Depcrate_teststest_into_vec {
() => {
// Module: crate::tests
// Provides: {"test_into_vec"}
// Dependencies: {}
# [test] fn test_into_vec () { let vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 2) ; assert_eq ! (vec . into_vec () , vec ! [0 , 1]) ; let vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) ; assert_eq ! (vec . into_vec () , vec ! [0 , 1 , 2]) ; }
};
}
