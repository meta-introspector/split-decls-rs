// Generated macro for bytes_of_slice (function)
macro_rules! Depcrate_podbytes_of_slice {
() => {
// Module: crate::pod
// Provides: {"bytes_of_slice"}
// Dependencies: {}
# [doc = " Cast a slice of a `Pod` type to a byte slice."] # [inline] pub fn bytes_of_slice < T : Pod > (val : & [T]) -> & [u8] { let size = val . len () . wrapping_mul (mem :: size_of :: < T > ()) ; unsafe { slice :: from_raw_parts (val . as_ptr () . cast () , size) } }
};
}
