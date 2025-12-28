macro_rules! hyper_body_data_callback {
    () => {
        type hyper_body_data_callback = extern "C" fn (* mut c_void , * mut hyper_context < '_ > , * mut * mut hyper_buf) -> c_int ;
    };
}

hyper_body_data_callback!()