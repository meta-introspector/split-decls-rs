// Generated macro for gzfwrite (function)
macro_rules! Depcrate_gzgzfwrite {
() => {
// Module: crate::gz
// Provides: {"gzfwrite"}
// Dependencies: {}
# [doc = " Compress and write `nitems` items of size `size` from `buf` to `file`, duplicating"] # [doc = " the interface of C stdio's `fwrite`, with `size_t` request and return types."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - The number of full items written of size `size` on success."] # [doc = " - Zero on error."] # [doc = ""] # [doc = " Note: If the multiplication of `size` and `nitems` overflows, i.e. the product does"] # [doc = " not fit in a `size_t`, then nothing is written, zero is returned, and the error state"] # [doc = " is set to `Z_STREAM_ERROR`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [doc = " - The caller must ensure that `buf` points to at least `size * nitems` readable bytes."] # [export_name = crate :: prefix ! (gzfwrite)] pub unsafe extern "C-unwind" fn gzfwrite (buf : * const c_void , size : size_t , nitems : size_t , file : gzFile ,) -> size_t { if size == 0 || buf . is_null () { return 0 ; } let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return 0 ; } ; if state . mode != GzMode :: GZ_WRITE || state . err != Z_OK { return 0 ; } let Some (len) = size . checked_mul (nitems) else { const MSG : & str = "request does not fit in a size_t" ; unsafe { gz_error (state , Some ((Z_STREAM_ERROR , MSG))) } ; return 0 ; } ; if len == 0 { len } else { (unsafe { gz_write (state , buf , len) }) as size_t / size } }
};
}
