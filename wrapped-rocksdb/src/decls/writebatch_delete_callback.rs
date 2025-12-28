macro_rules! deps {
    () => {
        WriteBatchIterator!();
    };
}

macro_rules! writebatch_delete_callback {
    () => {
        deps!();
        unsafe extern "C" fn writebatch_delete_callback < T : WriteBatchIterator > (state : * mut c_void , k : * const c_char , klen : usize ,) { unsafe { let callbacks = & mut * (state as * mut T) ; let key = slice :: from_raw_parts (k as * const u8 , klen) ; callbacks . delete (key) ; } }
    };
}

writebatch_delete_callback!()