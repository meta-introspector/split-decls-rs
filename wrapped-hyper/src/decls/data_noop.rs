macro_rules! data_noop {
    () => {
        # [doc = " cbindgen:ignore"] extern "C" fn data_noop (_userdata : * mut c_void , _ : * mut hyper_context < '_ > , _ : * mut * mut hyper_buf ,) -> c_int { super :: task :: HYPER_POLL_READY }
    };
}

data_noop!()