// Generated macro for gz_avail (function)
macro_rules! Depcrate_gzgz_avail {
() => {
// Module: crate::gz
// Provides: {"gz_avail"}
// Dependencies: {}
unsafe fn gz_avail (state : & mut GzState) -> Result < usize , () > { if state . err != Z_OK && state . err != Z_BUF_ERROR { return Err (()) ; } if ! state . eof { if state . stream . avail_in != 0 { unsafe { ptr :: copy (state . stream . next_in , state . input , state . stream . avail_in as usize ,) } ; } let got = unsafe { gz_load (state , state . input . add (state . stream . avail_in as usize) , state . in_size - state . stream . avail_in as usize ,) } ? ; state . stream . avail_in += got as uInt ; state . stream . next_in = state . input ; } Ok (state . stream . avail_in as usize) }
};
}
