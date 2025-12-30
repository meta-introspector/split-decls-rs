// Generated macro for writebatch_put_cf_callback (function)
macro_rules! Depcrate_write_batchwritebatch_put_cf_callback {
() => {
// Module: crate::write_batch
// Provides: {"writebatch_put_cf_callback"}
// Dependencies: {}
unsafe extern "C" fn writebatch_put_cf_callback < T : WriteBatchIteratorCf > (state : * mut c_void , cfid : u32 , k : * const c_char , klen : usize , v : * const c_char , vlen : usize ,) { unsafe { let callbacks = & mut * (state as * mut T) ; let key = slice :: from_raw_parts (k as * const u8 , klen) ; let value = slice :: from_raw_parts (v as * const u8 , vlen) ; callbacks . put_cf (cfid , key , value) ; } }
};
}
