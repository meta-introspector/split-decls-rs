// Generated macro for test_blocked_index_oob (function)
macro_rules! Depcrate_slice_blockedtest_blocked_index_oob {
() => {
// Module: crate::slice::blocked
// Provides: {"test_blocked_index_oob"}
// Dependencies: {}
# [should_panic] # [test] fn test_blocked_index_oob () { let data = [0 , 1 , 2 , 3 , 4] ; let iter = BlockedIter :: < [u32 ; 2] , _ > :: from_slice (& data) ; iter [2] ; }
};
}
