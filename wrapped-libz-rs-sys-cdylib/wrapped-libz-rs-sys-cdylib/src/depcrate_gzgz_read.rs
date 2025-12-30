// Generated macro for gz_read (function)
macro_rules! Depcrate_gzgz_read {
() => {
// Module: crate::gz
// Provides: {"gz_read"}
// Dependencies: {}
unsafe fn gz_read (state : & mut GzState , mut buf : * mut u8 , mut len : usize) -> usize { if len == 0 { return 0 ; } if state . seek { state . seek = false ; if gz_skip (state , state . skip) . is_err () { return 0 ; } } let mut got = 0 ; loop { let mut n = Ord :: min (len , c_uint :: MAX as usize) ; if state . have != 0 { n = Ord :: min (n , state . have as usize) ; unsafe { ptr :: copy_nonoverlapping (state . next , buf , n) } ; state . next = unsafe { state . next . add (n) } ; state . have -= n as c_uint ; } else if state . eof && state . stream . avail_in == 0 { state . past = true ; break ; } else if state . how == How :: Look || n < state . in_size * 2 { if unsafe { gz_fetch (state) } . is_err () { return 0 ; } continue ; } else if state . how == How :: Copy { let Ok (bytes_read) = (unsafe { gz_load (state , buf , n) }) else { return 0 ; } ; n = bytes_read ; } else { debug_assert_eq ! (state . how , How :: Gzip) ; state . stream . avail_out = n as c_uint ; state . stream . next_out = buf ; if unsafe { gz_decomp (state) } . is_err () { return 0 ; } n = state . have as usize ; state . have = 0 ; } len -= n ; buf = unsafe { buf . add (n) } ; got += n ; state . pos += n as i64 ; if len == 0 { break ; } } got }
};
}
