macro_rules! deps {
    () => {
        TransformCallback!();
    };
}

macro_rules! transform_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn transform_callback (raw_cb : * mut c_void , raw_key : * const c_char , key_len : size_t , dst_length : * mut size_t ,) -> * mut c_char { let cb = unsafe { & mut * (raw_cb as * mut TransformCallback) } ; let key = unsafe { slice :: from_raw_parts (raw_key as * const u8 , key_len) } ; let prefix = (cb . transform_fn) (key) ; unsafe { * dst_length = prefix . len () as size_t } ; prefix . as_ptr () as * mut c_char }
    };
}

transform_callback!();