// Generated macro for test_into_iter_as_slice (function)
macro_rules! Depcrate_teststest_into_iter_as_slice {
() => {
// Module: crate::tests
// Provides: {"test_into_iter_as_slice"}
// Dependencies: {}
# [test] fn test_into_iter_as_slice () { let vec = SmallVec :: < u32 , 2 > :: from (& [1 , 2 , 3] [..]) ; let mut iter = vec . clone () . into_iter () ; assert_eq ! (iter . as_slice () , & [1 , 2 , 3]) ; assert_eq ! (iter . as_mut_slice () , & [1 , 2 , 3]) ; iter . next () ; assert_eq ! (iter . as_slice () , & [2 , 3]) ; assert_eq ! (iter . as_mut_slice () , & [2 , 3]) ; iter . next_back () ; assert_eq ! (iter . as_slice () , & [2]) ; assert_eq ! (iter . as_mut_slice () , & [2]) ; }
};
}
