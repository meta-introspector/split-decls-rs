// Generated macro for gz_decomp (function)
macro_rules! Depcrate_gzgz_decomp {
() => {
// Module: crate::gz
// Provides: {"gz_decomp"}
// Dependencies: {}
unsafe fn gz_decomp (state : & mut GzState) -> Result < () , () > { let had = state . stream . avail_out ; loop { if state . stream . avail_in == 0 && unsafe { gz_avail (state) } . is_err () { return Err (()) ; } if state . stream . avail_in == 0 { unsafe { gz_error (state , Some ((Z_BUF_ERROR , "unexpected end of file"))) } ; break ; } match unsafe { inflate (& mut state . stream , Z_NO_FLUSH) } { Z_STREAM_ERROR | Z_NEED_DICT => { const MSG : & str = "internal error: inflate stream corrupt" ; unsafe { gz_error (state , Some ((Z_STREAM_ERROR , MSG))) } ; return Err (()) ; } Z_MEM_ERROR => { unsafe { gz_error (state , Some ((Z_MEM_ERROR , "out of memory"))) } ; return Err (()) ; } Z_DATA_ERROR => { unsafe { gz_error (state , Some ((Z_DATA_ERROR , "compressed data error"))) } ; return Err (()) ; } Z_STREAM_END => { state . how = How :: Look ; break ; } _ => { } } if state . stream . avail_out == 0 { break ; } } state . have = had - state . stream . avail_out ; state . next = unsafe { state . stream . next_out . sub (state . have as usize) } ; Ok (()) }
};
}
