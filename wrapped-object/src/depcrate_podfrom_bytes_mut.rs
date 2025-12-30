// Generated macro for from_bytes_mut (function)
macro_rules! Depcrate_podfrom_bytes_mut {
() => {
// Module: crate::pod
// Provides: {"from_bytes_mut"}
// Dependencies: {}
# [doc = " Cast the head of a mutable byte slice to a `Pod` type."] # [doc = ""] # [doc = " Returns the type and the tail of the byte slice."] # [doc = ""] # [doc = " Returns an error if the byte slice is too short or the alignment is invalid."] # [inline] pub fn from_bytes_mut < T : Pod > (data : & mut [u8]) -> Result < (& mut T , & mut [u8]) > { let size = mem :: size_of :: < T > () ; if size > data . len () { return Err (()) ; } let (data , tail) = data . split_at_mut (size) ; let ptr = data . as_mut_ptr () ; if (ptr as usize) % mem :: align_of :: < T > () != 0 { return Err (()) ; } let val = unsafe { & mut * ptr . cast () } ; Ok ((val , tail)) }
};
}
