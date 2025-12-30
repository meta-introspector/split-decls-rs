// Generated macro for gzputs (function)
macro_rules! Depcrate_gzgzputs {
() => {
// Module: crate::gz
// Provides: {"gzputs"}
// Dependencies: {}
# [doc = " Compress and write the given null-terminated string `s` to file, excluding"] # [doc = " the terminating null character."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - the number of characters written, on success."] # [doc = " - `-1` in case of error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [doc = " - `s` must point to a null-terminated C string."] # [export_name = crate :: prefix ! (gzputs)] pub unsafe extern "C-unwind" fn gzputs (file : gzFile , s : * const c_char) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return - 1 ; } ; if s . is_null () { return - 1 ; } if state . mode != GzMode :: GZ_WRITE || state . err != Z_OK { return - 1 ; } let len = unsafe { libc :: strlen (s) } ; if c_int :: try_from (len) . is_err () { const MSG : & str = "string length does not fit in int" ; unsafe { gz_error (state , Some ((Z_STREAM_ERROR , MSG))) } ; return - 1 ; } let put = unsafe { gz_write (state , s . cast :: < c_void > () , len) } ; match put . cmp (& (len as i32)) { Ordering :: Less => - 1 , Ordering :: Equal | Ordering :: Greater => len as _ , } }
};
}
