macro_rules! write_noop {
    () => {
        # [doc = " cbindgen:ignore"] extern "C" fn write_noop (_userdata : * mut c_void , _ : * mut hyper_context < '_ > , _buf : * const u8 , _buf_len : size_t ,) -> size_t { 0 }
    };
}

write_noop!();