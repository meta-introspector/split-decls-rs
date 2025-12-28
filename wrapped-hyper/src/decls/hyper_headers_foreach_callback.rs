macro_rules! hyper_headers_foreach_callback {
    () => {
        type hyper_headers_foreach_callback = extern "C" fn (* mut c_void , * const u8 , size_t , * const u8 , size_t) -> c_int ;
    };
}

hyper_headers_foreach_callback!();