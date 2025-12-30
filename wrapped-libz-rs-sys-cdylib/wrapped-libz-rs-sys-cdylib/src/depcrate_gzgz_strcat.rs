// Generated macro for gz_strcat (function)
macro_rules! Depcrate_gzgz_strcat {
() => {
// Module: crate::gz
// Provides: {"gz_strcat"}
// Dependencies: {}
unsafe fn gz_strcat (strings : & [& str]) -> * mut c_char { let mut len = 1 ; for src in strings { len += src . len () ; } let Some (buf) = ALLOCATOR . allocate_slice_raw :: < c_char > (len) else { return ptr :: null_mut () ; } ; let start = buf . as_ptr () . cast :: < c_char > () ; let mut dst = start . cast :: < u8 > () ; for src in strings { let size = src . len () ; unsafe { ptr :: copy_nonoverlapping (src . as_ptr () , dst , size) ; } ; dst = unsafe { dst . add (size) } ; } unsafe { * dst = 0 } ; start }
};
}
