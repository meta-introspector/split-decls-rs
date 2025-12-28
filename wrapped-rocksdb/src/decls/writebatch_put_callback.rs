macro_rules! deps {
    () => {
        WriteBatchIterator!();
    };
}

macro_rules! writebatch_put_callback {
    () => {
        deps!();
        unsafe extern "C" fn writebatch_put_callback < T : WriteBatchIterator > (state : * mut c_void , k : * const c_char , klen : usize , v : * const c_char , vlen : usize ,) { unsafe { let callbacks = & mut * (state as * mut T) ; let key = slice :: from_raw_parts (k as * const u8 , klen) ; let value = slice :: from_raw_parts (v as * const u8 , vlen) ; callbacks . put (key , value) ; } }
    };
}

writebatch_put_callback!()