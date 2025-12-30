// Generated macro for slice_from_bytes (function)
macro_rules! Depcrate_podslice_from_bytes {
() => {
// Module: crate::pod
// Provides: {"slice_from_bytes"}
// Dependencies: {}
# [doc = " Cast the head of a byte slice to a slice of a `Pod` type."] # [doc = ""] # [doc = " Returns the type slice and the tail of the byte slice."] # [doc = ""] # [doc = " Returns an error if the byte slice is too short or the alignment is invalid."] # [inline] pub fn slice_from_bytes < T : Pod > (data : & [u8] , count : usize) -> Result < (& [T] , & [u8]) > { let size = count . checked_mul (mem :: size_of :: < T > ()) . ok_or (()) ? ; let tail = data . get (size ..) . ok_or (()) ? ; let ptr = data . as_ptr () ; if (ptr as usize) % mem :: align_of :: < T > () != 0 { return Err (()) ; } let slice = unsafe { slice :: from_raw_parts (ptr . cast () , count) } ; Ok ((slice , tail)) }
};
}
