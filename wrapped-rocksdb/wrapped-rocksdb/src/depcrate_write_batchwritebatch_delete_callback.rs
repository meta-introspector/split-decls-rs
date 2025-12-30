// Generated macro for writebatch_delete_callback (function)
macro_rules! Depcrate_write_batchwritebatch_delete_callback {
() => {
// Module: crate::write_batch
// Provides: {"writebatch_delete_callback"}
// Dependencies: {}
unsafe extern "C" fn writebatch_delete_callback < T : WriteBatchIterator > (state : * mut c_void , k : * const c_char , klen : usize ,) { unsafe { let callbacks = & mut * (state as * mut T) ; let key = slice :: from_raw_parts (k as * const u8 , klen) ; callbacks . delete (key) ; } }
};
}
