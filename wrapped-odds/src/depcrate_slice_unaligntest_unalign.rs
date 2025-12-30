// Generated macro for test_unalign (function)
macro_rules! Depcrate_slice_unaligntest_unalign {
() => {
// Module: crate::slice::unalign
// Provides: {"test_unalign"}
// Dependencies: {}
# [test] fn test_unalign () { let data = [0u8 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9] ; let mut iter = UnalignedIter :: < u32 > :: from_slice (& data) ; assert_eq ! (iter . next () , Some (u32 :: from_be (0x00010203))) ; assert_eq ! (iter . next () , Some (u32 :: from_be (0x04050607))) ; let mut tail = iter . tail () ; assert_eq ! (tail . next () , Some (8)) ; assert_eq ! (tail . next () , Some (9)) ; assert_eq ! (tail . next () , None) ; }
};
}
