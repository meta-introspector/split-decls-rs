// Generated macro for gzvprintf (function)
macro_rules! Depcrate_gzgzvprintf {
() => {
// Module: crate::gz
// Provides: {"gzvprintf"}
// Dependencies: {}
# [cfg (feature = "gzprintf")] unsafe extern "C-unwind" fn gzvprintf (file : gzFile , format : * const c_char , va : core :: ffi :: VaList ,) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return Z_STREAM_ERROR ; } ; if state . mode != GzMode :: GZ_WRITE || state . err != Z_OK { return Z_STREAM_ERROR ; } if state . input . is_null () && gz_init (state) . is_err () { return state . err ; } if state . seek { state . seek = false ; if gz_zero (state , state . skip as _) . is_err () { return state . err ; } } if state . stream . avail_in == 0 { state . stream . next_in = state . input ; } let next = unsafe { (state . stream . next_in) . add (state . stream . avail_in as usize) } . cast_mut () ; extern "C" { fn vsnprintf (s : * mut c_char , n : libc :: size_t , format : * const c_char , va : core :: ffi :: VaList ,) -> c_int ; } let len = unsafe { vsnprintf (next . cast :: < c_char > () , state . in_size , format , va) } ; if len == 0 || len as usize >= state . in_size { return 0 ; } state . stream . avail_in += len as u32 ; state . pos += i64 :: from (len) ; if state . stream . avail_in as usize >= state . in_size { let left = state . stream . avail_in - state . in_size as u32 ; state . stream . avail_in = state . in_size as u32 ; if gz_comp (state , Z_NO_FLUSH) . is_err () { return state . err ; } unsafe { core :: ptr :: copy (state . input . add (state . in_size) , state . input , left as usize) } ; state . stream . next_in = state . input ; state . stream . avail_in = left ; } len }
};
}
