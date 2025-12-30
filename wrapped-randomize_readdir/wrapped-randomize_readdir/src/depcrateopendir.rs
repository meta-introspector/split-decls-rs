// Generated macro for opendir (function)
macro_rules! Depcrateopendir {
() => {
// Module: crate
// Provides: {"opendir"}
// Dependencies: {}
# [doc = " Opens a directory stream for reading."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe because:"] # [doc = " - `dirname` must be a valid pointer to a null-terminated C string"] # [doc = " - The caller is responsible for eventually closing the returned directory stream with `closedir`"] # [no_mangle] pub unsafe extern "C" fn opendir (dirname : * const c_char) -> * mut DIR { let state = STATE . wait () ; let dirp = unsafe { (state . opendir) (dirname) } ; info ! ("{:p}: opening directory '{}'" , dirp , unsafe { CStr :: from_ptr (dirname) } . to_string_lossy ()) ; if ! dirp . is_null () { state . new_opendir (dirp) ; } dirp }
};
}
