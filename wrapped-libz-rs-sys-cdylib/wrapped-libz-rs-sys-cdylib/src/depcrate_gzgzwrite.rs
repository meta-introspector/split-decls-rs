// Generated macro for gzwrite (function)
macro_rules! Depcrate_gzgzwrite {
() => {
// Module: crate::gz
// Provides: {"gzwrite"}
// Dependencies: {}
# [doc = " Compress and write the len uncompressed bytes at buf to file."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - The number of uncompressed bytes written, on success."] # [doc = " - Or 0 in case of error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [doc = " - `buf` must point to at least `len` bytes of readable memory."] # [export_name = crate :: prefix ! (gzwrite)] pub unsafe extern "C-unwind" fn gzwrite (file : gzFile , buf : * const c_void , len : c_uint) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return 0 ; } ; if state . mode != GzMode :: GZ_WRITE || state . err != Z_OK { return 0 ; } if c_int :: try_from (len) . is_err () { const MSG : & str = "requested length does not fit in int" ; unsafe { gz_error (state , Some ((Z_DATA_ERROR , MSG))) } ; return 0 ; } let Ok (len) = usize :: try_from (len) else { const MSG : & str = "requested length does not fit in usize" ; unsafe { gz_error (state , Some ((Z_DATA_ERROR , MSG))) } ; return 0 ; } ; unsafe { gz_write (state , buf , len) } }
};
}
