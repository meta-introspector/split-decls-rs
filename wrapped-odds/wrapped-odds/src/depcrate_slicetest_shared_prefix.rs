// Generated macro for test_shared_prefix (function)
macro_rules! Depcrate_slicetest_shared_prefix {
() => {
// Module: crate::slice
// Provides: {"test_shared_prefix"}
// Dependencies: {}
# [test] fn test_shared_prefix () { let mut a = [0xff ; 256] ; let b = [0xff ; 256] ; for byte in 0 .. 255 { for i in 0 .. a . len () { a [i] = byte ; let ans = shared_prefix (& a , & b) ; assert ! (ans == i , "failed for index {} and byte {:x} (got ans={})" , i , byte , ans) ; a [i] = 0xff ; } } }
};
}
