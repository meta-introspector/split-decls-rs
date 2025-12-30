// Generated macro for test_rev_slice_find (function)
macro_rules! Depcrate_slice_revtest_rev_slice_find {
() => {
// Module: crate::slice::rev
// Provides: {"test_rev_slice_find"}
// Dependencies: {}
# [test] fn test_rev_slice_find () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; for (i , elt) in r . into_iter () . enumerate () { assert_eq ! (r . find (elt) , Some (i)) ; } for (i , elt) in r . into_iter () . enumerate () { assert_eq ! (r . rfind (elt) , Some (i)) ; } }
};
}
