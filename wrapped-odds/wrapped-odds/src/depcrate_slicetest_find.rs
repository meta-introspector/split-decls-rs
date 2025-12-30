// Generated macro for test_find (function)
macro_rules! Depcrate_slicetest_find {
() => {
// Module: crate::slice
// Provides: {"test_find"}
// Dependencies: {}
# [test] fn test_find () { let v = [0 , 1 , 7 , 0 , 0 , 2 , 3 , 5 , 1 , 5 , 3 , 1 , 2 , 1] ; assert_eq ! (v . find_split (& 7) , v . split_at (2)) ; assert_eq ! (v . rfind_split (& 7) , v . split_at (2)) ; assert_eq ! (v . rfind_split (& 2) , v . split_at (v . len () - 2)) ; }
};
}
