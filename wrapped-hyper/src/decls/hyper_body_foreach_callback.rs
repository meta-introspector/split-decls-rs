macro_rules! hyper_body_foreach_callback {
    () => {
        type hyper_body_foreach_callback = extern "C" fn (* mut c_void , * const hyper_buf) -> c_int ;
    };
}

hyper_body_foreach_callback!()