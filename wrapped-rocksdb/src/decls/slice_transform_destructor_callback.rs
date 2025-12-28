macro_rules! deps {
    () => {
        TransformCallback!();
    };
}

macro_rules! slice_transform_destructor_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn slice_transform_destructor_callback (raw_cb : * mut c_void) { drop (unsafe { Box :: from_raw (raw_cb as * mut TransformCallback) }) ; }
    };
}

slice_transform_destructor_callback!()