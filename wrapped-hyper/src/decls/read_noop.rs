macro_rules! read_noop {
    () => {
        # [doc = " cbindgen:ignore"] extern "C" fn read_noop (_userdata : * mut c_void , _ : * mut hyper_context < '_ > , _buf : * mut u8 , _buf_len : size_t ,) -> size_t { 0 }
    };
}

read_noop!()