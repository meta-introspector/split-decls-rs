// Generated macro for load_u64 (function)
macro_rules! Depcrate_sliceload_u64 {
() => {
// Module: crate::slice
// Provides: {"load_u64"}
// Dependencies: {}
# [doc = " Unaligned load of a u64 at index `i` in `buf`"] unsafe fn load_u64 (buf : & [u8] , i : usize) -> u64 { debug_assert ! (i + 8 <= buf . len ()) ; let mut data = 0u64 ; ptr :: copy_nonoverlapping (get_unchecked (buf , i) , & mut data as * mut _ as * mut u8 , 8) ; data }
};
}
