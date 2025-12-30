// Generated macro for slice_from_all_bytes_mut (function)
macro_rules! Depcrate_podslice_from_all_bytes_mut {
() => {
// Module: crate::pod
// Provides: {"slice_from_all_bytes_mut"}
// Dependencies: {}
# [doc = " Cast all of a byte slice to a slice of a `Pod` type."] # [doc = ""] # [doc = " Returns the type slice."] # [doc = ""] # [doc = " Returns an error if the size of the byte slice is not an exact multiple"] # [doc = " of the type size, or the alignment is invalid."] # [inline] pub fn slice_from_all_bytes_mut < T : Pod > (data : & mut [u8]) -> Result < & mut [T] > { let count = data . len () / mem :: size_of :: < T > () ; let (slice , tail) = slice_from_bytes_mut (data , count) ? ; if ! tail . is_empty () { return Err (()) ; } Ok (slice) }
};
}
