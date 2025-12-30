// Generated macro for test_rev_slice_split (function)
macro_rules! Depcrate_slice_revtest_rev_slice_split {
() => {
// Module: crate::slice::rev
// Provides: {"test_rev_slice_split"}
// Dependencies: {}
# [test] fn test_rev_slice_split () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; for i in 0 .. r . len () { let (a , b) = r . split_at (i) ; assert_eq ! (a , & r [.. i]) ; assert_eq ! (b , & r [i ..]) ; } }
};
}
