macro_rules! add_certs_to_context {
    () => {
        # [cfg (not (target_env = "msvc"))] pub fn add_certs_to_context (_ : * mut c_void) { }
    };
}

add_certs_to_context!();