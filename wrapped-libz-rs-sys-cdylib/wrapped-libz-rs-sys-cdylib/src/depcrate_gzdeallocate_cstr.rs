// Generated macro for deallocate_cstr (function)
macro_rules! Depcrate_gzdeallocate_cstr {
() => {
// Module: crate::gz
// Provides: {"deallocate_cstr"}
// Dependencies: {}
unsafe fn deallocate_cstr (s : * mut c_char) { if s . is_null () { return ; } unsafe { ALLOCATOR . deallocate :: < c_char > (s , libc :: strlen (s) + 1) } ; }
};
}
