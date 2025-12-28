macro_rules! deps {
    () => {
        ComparatorWithTsCallback!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl ComparatorWithTsCallback { pub unsafe extern "C" fn destructor_callback (raw_cb : * mut c_void) { drop (unsafe { Box :: from_raw (raw_cb as * mut Self) }) ; } pub unsafe extern "C" fn name_callback (raw_cb : * mut c_void) -> * const c_char { let cb : & mut Self = unsafe { & mut * (raw_cb as * mut Self) } ; let ptr = cb . name . as_ptr () ; ptr as * const c_char } pub unsafe extern "C" fn compare_callback (raw_cb : * mut c_void , a_raw : * const c_char , a_len : size_t , b_raw : * const c_char , b_len : size_t ,) -> c_int { let cb : & mut Self = unsafe { & mut * (raw_cb as * mut Self) } ; let a : & [u8] = unsafe { slice :: from_raw_parts (a_raw as * const u8 , a_len) } ; let b : & [u8] = unsafe { slice :: from_raw_parts (b_raw as * const u8 , b_len) } ; (cb . compare_fn) (a , b) as c_int } pub unsafe extern "C" fn compare_ts_callback (raw_cb : * mut c_void , a_ts_raw : * const c_char , a_ts_len : size_t , b_ts_raw : * const c_char , b_ts_len : size_t ,) -> c_int { let cb : & mut Self = unsafe { & mut * (raw_cb as * mut Self) } ; let a_ts : & [u8] = unsafe { slice :: from_raw_parts (a_ts_raw as * const u8 , a_ts_len) } ; let b_ts : & [u8] = unsafe { slice :: from_raw_parts (b_ts_raw as * const u8 , b_ts_len) } ; (cb . compare_ts_fn) (a_ts , b_ts) as c_int } pub unsafe extern "C" fn compare_without_ts_callback (raw_cb : * mut c_void , a_raw : * const c_char , a_len : size_t , a_has_ts_raw : c_uchar , b_raw : * const c_char , b_len : size_t , b_has_ts_raw : c_uchar ,) -> c_int { let cb : & mut Self = unsafe { & mut * (raw_cb as * mut Self) } ; let a : & [u8] = unsafe { slice :: from_raw_parts (a_raw as * const u8 , a_len) } ; let a_has_ts = a_has_ts_raw != 0 ; let b : & [u8] = unsafe { slice :: from_raw_parts (b_raw as * const u8 , b_len) } ; let b_has_ts = b_has_ts_raw != 0 ; (cb . compare_without_ts_fn) (a , a_has_ts , b , b_has_ts) as c_int } }
    };
}

impl_90!()