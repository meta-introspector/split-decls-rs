// Generated macro for writebatch_delete_cf_callback (function)
macro_rules! Depcrate_write_batchwritebatch_delete_cf_callback {
() => {
// Module: crate::write_batch
// Provides: {"writebatch_delete_cf_callback"}
// Dependencies: {}
unsafe extern "C" fn writebatch_delete_cf_callback < T : WriteBatchIteratorCf > (state : * mut c_void , cfid : u32 , k : * const c_char , klen : usize ,) { unsafe { let callbacks = & mut * (state as * mut T) ; let key = slice :: from_raw_parts (k as * const u8 , klen) ; callbacks . delete_cf (cfid , key) ; } }
};
}
