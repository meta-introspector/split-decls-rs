// Generated macro for test_blocked_index (function)
macro_rules! Depcrate_slice_blockedtest_blocked_index {
() => {
// Module: crate::slice::blocked
// Provides: {"test_blocked_index"}
// Dependencies: {}
# [test] fn test_blocked_index () { let data = [0 , 1 , 2 , 3 , 4] ; let iter = BlockedIter :: < [u32 ; 2] , _ > :: from_slice (& data) ; assert_eq ! (iter [0] , [0 , 1]) ; assert_eq ! (iter [1] , [2 , 3]) ; }
};
}
