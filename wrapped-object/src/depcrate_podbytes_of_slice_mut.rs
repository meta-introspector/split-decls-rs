// Generated macro for bytes_of_slice_mut (function)
macro_rules! Depcrate_podbytes_of_slice_mut {
() => {
// Module: crate::pod
// Provides: {"bytes_of_slice_mut"}
// Dependencies: {}
# [doc = " Cast a slice of a `Pod` type to a mutable byte slice."] # [inline] pub fn bytes_of_slice_mut < T : Pod > (val : & mut [T]) -> & mut [u8] { let size = val . len () . wrapping_mul (mem :: size_of :: < T > ()) ; unsafe { slice :: from_raw_parts_mut (val . as_mut_ptr () . cast () , size) } }
};
}
