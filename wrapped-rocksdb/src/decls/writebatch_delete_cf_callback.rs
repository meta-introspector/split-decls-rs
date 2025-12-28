macro_rules! deps {
    () => {
        WriteBatchIteratorCf!();
    };
}

macro_rules! writebatch_delete_cf_callback {
    () => {
        deps!();
        unsafe extern "C" fn writebatch_delete_cf_callback < T : WriteBatchIteratorCf > (state : * mut c_void , cfid : u32 , k : * const c_char , klen : usize ,) { unsafe { let callbacks = & mut * (state as * mut T) ; let key = slice :: from_raw_parts (k as * const u8 , klen) ; callbacks . delete_cf (cfid , key) ; } }
    };
}

writebatch_delete_cf_callback!();