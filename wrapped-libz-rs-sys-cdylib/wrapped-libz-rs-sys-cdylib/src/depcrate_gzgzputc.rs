// Generated macro for gzputc (function)
macro_rules! Depcrate_gzgzputc {
() => {
// Module: crate::gz
// Provides: {"gzputc"}
// Dependencies: {}
# [doc = " Compress and write `c`, converted to an unsigned 8-bit char, into `file`."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = "  - The value that was written, on success."] # [doc = "  - `-1` on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzputc)] pub unsafe extern "C-unwind" fn gzputc (file : gzFile , c : c_int) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return - 1 ; } ; if state . mode != GzMode :: GZ_WRITE || state . err != Z_OK { return - 1 ; } if state . seek { state . seek = false ; if gz_zero (state , state . skip as _) . is_err () { return - 1 ; } } if ! state . input . is_null () { if state . stream . avail_in == 0 { state . stream . next_in = state . input ; } let have = unsafe { state . input_len () } ; if have < state . in_size { unsafe { * state . input . add (have) = c as u8 } ; state . stream . avail_in += 1 ; state . pos += 1 ; return c & 0xff ; } } let buf = [c as u8] ; match unsafe { gz_write (state , buf . as_ptr () . cast :: < c_void > () , 1) } { 1 => c & 0xff , _ => - 1 , } }
};
}
