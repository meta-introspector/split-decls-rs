// Generated macro for bytes_of_mut (function)
macro_rules! Depcrate_podbytes_of_mut {
() => {
// Module: crate::pod
// Provides: {"bytes_of_mut"}
// Dependencies: {}
# [doc = " Cast a `Pod` type to a mutable byte slice."] # [inline] pub fn bytes_of_mut < T : Pod > (val : & mut T) -> & mut [u8] { let size = mem :: size_of :: < T > () ; unsafe { slice :: from_raw_parts_mut (slice :: from_mut (val) . as_mut_ptr () . cast () , size) } }
};
}
