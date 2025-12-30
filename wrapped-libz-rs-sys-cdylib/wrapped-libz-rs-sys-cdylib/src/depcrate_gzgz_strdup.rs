// Generated macro for gz_strdup (function)
macro_rules! Depcrate_gzgz_strdup {
() => {
// Module: crate::gz
// Provides: {"gz_strdup"}
// Dependencies: {}
unsafe fn gz_strdup (src : * const c_char) -> * mut c_char { if src . is_null () { return ptr :: null_mut () ; } let src = unsafe { CStr :: from_ptr (src) } ; let len = src . to_bytes_with_nul () . len () ; let Some (dst) = ALLOCATOR . allocate_slice_raw :: < c_char > (len) else { return ptr :: null_mut () ; } ; unsafe { core :: ptr :: copy_nonoverlapping (src . as_ptr () , dst . as_ptr () , len) } ; dst . as_ptr () }
};
}
