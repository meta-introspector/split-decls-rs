macro_rules! delete_callback {
    () => {
        pub unsafe extern "C" fn delete_callback (_raw_cb : * mut c_void , value : * const c_char , value_length : size_t ,) { if ! value . is_null () { drop (unsafe { Box :: from_raw (slice :: from_raw_parts_mut (value as * mut u8 , value_length)) }) ; } }
    };
}

delete_callback!();