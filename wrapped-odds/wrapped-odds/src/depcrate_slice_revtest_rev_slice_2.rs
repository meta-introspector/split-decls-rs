// Generated macro for test_rev_slice_2 (function)
macro_rules! Depcrate_slice_revtest_rev_slice_2 {
() => {
// Module: crate::slice::rev
// Provides: {"test_rev_slice_2"}
// Dependencies: {}
# [should_panic] # [test] fn test_rev_slice_2 () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; r [4] ; }
};
}
