macro_rules! UserBody {
    () => {
        pub (crate) struct UserBody { data_func : hyper_body_data_callback , userdata : * mut c_void , }
    };
}

UserBody!();