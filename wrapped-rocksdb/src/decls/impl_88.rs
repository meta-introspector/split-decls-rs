macro_rules! deps {
    () => {
        ComparatorCallback!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl ComparatorCallback { pub unsafe extern "C" fn destructor_callback (raw_cb : * mut c_void) { drop (unsafe { Box :: from_raw (raw_cb as * mut Self) }) ; } pub unsafe extern "C" fn name_callback (raw_cb : * mut c_void) -> * const c_char { let cb : & mut Self = unsafe { & mut * (raw_cb as * mut Self) } ; let ptr = cb . name . as_ptr () ; ptr as * const c_char } pub unsafe extern "C" fn compare_callback (raw_cb : * mut c_void , a_raw : * const c_char , a_len : size_t , b_raw : * const c_char , b_len : size_t ,) -> c_int { let cb : & mut Self = unsafe { & mut * (raw_cb as * mut Self) } ; let a : & [u8] = unsafe { slice :: from_raw_parts (a_raw as * const u8 , a_len) } ; let b : & [u8] = unsafe { slice :: from_raw_parts (b_raw as * const u8 , b_len) } ; (cb . compare_fn) (a , b) as c_int } }
    };
}

impl_88!()