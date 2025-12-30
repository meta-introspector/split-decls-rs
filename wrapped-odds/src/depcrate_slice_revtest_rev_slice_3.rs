// Generated macro for test_rev_slice_3 (function)
macro_rules! Depcrate_slice_revtest_rev_slice_3 {
() => {
// Module: crate::slice::rev
// Provides: {"test_rev_slice_3"}
// Dependencies: {}
# [should_panic] # [test] fn test_rev_slice_3 () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; r [! 0] ; }
};
}
