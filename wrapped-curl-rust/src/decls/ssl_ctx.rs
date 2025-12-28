macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ssl_ctx {
    () => {
        deps!();
        pub fn ssl_ctx (cx : * mut c_void) -> Result < () , Error > { windows :: add_certs_to_context (cx) ; Ok (()) }
    };
}

ssl_ctx!()