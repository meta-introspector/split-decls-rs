// Generated macro for gzbuffer (function)
macro_rules! Depcrate_gzgzbuffer {
() => {
// Module: crate::gz
// Provides: {"gzbuffer"}
// Dependencies: {}
# [doc = " Set the internal buffer size used by this library's functions for `file` to"] # [doc = " `size`.  The default buffer size is 128 KB.  This function must be called"] # [doc = " after [`gzopen`] or [`gzdopen`], but before any other calls that read or write"] # [doc = " the file (including [`gzdirect`]).  The buffer memory allocation is always"] # [doc = " deferred to the first read or write.  Three times `size` in buffer space is"] # [doc = " allocated."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `0` on success."] # [doc = " * `-1` on failure."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `file` - file handle."] # [doc = " * `size` - requested buffer size in bytes."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file` must be one of the following:"] # [doc = " - A file handle must have been obtained from a function in this library, such as [`gzopen`]."] # [doc = " - A null pointer."] # [export_name = crate :: prefix ! (gzbuffer)] pub unsafe extern "C-unwind" fn gzbuffer (file : gzFile , size : c_uint) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return - 1 ; } ; if state . mode != GzMode :: GZ_READ && state . mode != GzMode :: GZ_WRITE { return - 1 ; } if state . in_size != 0 { return - 1 ; } let size = size as usize ; if size . checked_mul (2) . is_none () { return - 1 ; } state . want = Ord :: max (size , 8) ; 0 }
};
}
