macro_rules! deps {
    () => {
        TransformCallback!();
    };
}

macro_rules! slice_transform_name_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn slice_transform_name_callback (raw_cb : * mut c_void) -> * const c_char { let cb = unsafe { & mut * (raw_cb as * mut TransformCallback) } ; cb . name . as_ptr () }
    };
}

slice_transform_name_callback!()