// Generated macro for gz_write (function)
macro_rules! Depcrate_gzgz_write {
() => {
// Module: crate::gz
// Provides: {"gz_write"}
// Dependencies: {}
# [doc = " - `state` must have been properly initialized, e.g. by [`gzopen_help`]."] # [doc = " - `buf` must point to at least `len` bytes of readable memory."] unsafe fn gz_write (state : & mut GzState , mut buf : * const c_void , mut len : usize) -> c_int { if len == 0 { return 0 ; } if state . input . is_null () && gz_init (state) . is_err () { return 0 ; } if state . seek { state . seek = false ; if gz_zero (state , state . skip as _) . is_err () { return 0 ; } } let put = len as c_int ; if len < state . in_size { loop { if state . stream . avail_in == 0 { state . stream . next_in = state . input ; } let have = unsafe { state . input_len () } ; let copy = Ord :: min (state . in_size . saturating_sub (have) , len) ; unsafe { ptr :: copy (buf , state . input . add (have) . cast :: < c_void > () , copy) } ; state . stream . avail_in += copy as c_uint ; state . pos += copy as i64 ; buf = unsafe { buf . add (copy) } ; len -= copy ; if len != 0 && gz_comp (state , Z_NO_FLUSH) . is_err () { return 0 ; } if len == 0 { break ; } } } else { if state . stream . avail_in != 0 && gz_comp (state , Z_NO_FLUSH) . is_err () { return 0 ; } let save_next_in = state . stream . next_in ; state . stream . next_in = buf . cast :: < _ > () ; loop { let n = Ord :: min (len , c_uint :: MAX as usize) as c_uint ; state . stream . avail_in = n ; state . pos += n as i64 ; if gz_comp (state , Z_NO_FLUSH) . is_err () { return 0 ; } len -= n as usize ; if len == 0 { break ; } } state . stream . next_in = save_next_in ; } put }
};
}
