// Generated macro for gz_fetch (function)
macro_rules! Depcrate_gzgz_fetch {
() => {
// Module: crate::gz
// Provides: {"gz_fetch"}
// Dependencies: {}
unsafe fn gz_fetch (state : & mut GzState) -> Result < () , () > { loop { match & state . how { How :: Look => { unsafe { gz_look (state) } ? ; if state . how == How :: Look { return Ok (()) ; } } How :: Copy => { let bytes_read = unsafe { gz_load (state , state . output , state . out_size) } ? ; state . next = state . output ; state . have += bytes_read as uInt ; return Ok (()) ; } How :: Gzip => { state . stream . avail_out = state . out_size as c_uint ; state . stream . next_out = state . output ; unsafe { gz_decomp (state) } ? ; } } if state . have != 0 || (state . eof && state . stream . avail_in == 0) { break ; } } Ok (()) }
};
}
