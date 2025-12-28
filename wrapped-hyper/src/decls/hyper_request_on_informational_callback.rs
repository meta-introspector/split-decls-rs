macro_rules! hyper_request_on_informational_callback {
    () => {
        type hyper_request_on_informational_callback = extern "C" fn (* mut c_void , * mut hyper_response) ;
    };
}

hyper_request_on_informational_callback!();