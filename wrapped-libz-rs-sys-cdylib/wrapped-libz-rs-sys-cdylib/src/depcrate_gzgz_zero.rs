// Generated macro for gz_zero (function)
macro_rules! Depcrate_gzgz_zero {
() => {
// Module: crate::gz
// Provides: {"gz_zero"}
// Dependencies: {}
fn gz_zero (state : & mut GzState , mut len : usize) -> Result < () , () > { if state . stream . avail_in != 0 && gz_comp (state , Z_NO_FLUSH) . is_err () { return Err (()) ; } let mut first = true ; while len != 0 { let n = Ord :: min (state . in_size , len) ; if first { unsafe { state . input . write_bytes (0u8 , n) } ; first = false ; } state . stream . avail_in = n as _ ; state . stream . next_in = state . input ; state . pos += n as i64 ; if gz_comp (state , Z_NO_FLUSH) . is_err () { return Err (()) ; } len -= n ; } Ok (()) }
};
}
