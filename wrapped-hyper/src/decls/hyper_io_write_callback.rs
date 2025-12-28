macro_rules! hyper_io_write_callback {
    () => {
        type hyper_io_write_callback = extern "C" fn (* mut c_void , * mut hyper_context < '_ > , * const u8 , size_t) -> size_t ;
    };
}

hyper_io_write_callback!();