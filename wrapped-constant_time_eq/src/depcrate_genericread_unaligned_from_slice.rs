// Generated macro for read_unaligned_from_slice (function)
macro_rules! Depcrate_genericread_unaligned_from_slice {
() => {
// Module: crate::generic
// Provides: {"read_unaligned_from_slice"}
// Dependencies: {}
# [doc = " Equivalent to `read_unaligned` for byte slices."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All bit patterns must be valid for type T."] # [must_use] # [inline (always)] unsafe fn read_unaligned_from_slice < T > (src : & [u8]) -> T { assert_eq ! (src . len () , size_of ::< T > ()) ; unsafe { read_unaligned (src . as_ptr () . cast :: < T > ()) } }
};
}
