// Generated macro for test_rev_slice_1 (function)
macro_rules! Depcrate_slice_revtest_rev_slice_1 {
() => {
// Module: crate::slice::rev
// Provides: {"test_rev_slice_1"}
// Dependencies: {}
# [test] fn test_rev_slice_1 () { let data = [1 , 2 , 3 , 4] ; let rev = [4 , 3 , 2 , 1] ; assert_eq ! (<& RevSlice < _ >>:: from (& data [..]) , & rev [..]) ; assert ! (<& RevSlice < _ >>:: from (& data [..]) != & data [..]) ; let r = < & RevSlice < _ > > :: from (& data [..]) ; assert_eq ! (r [0] , rev [0]) ; assert_eq ! (r [3] , rev [3]) ; }
};
}
