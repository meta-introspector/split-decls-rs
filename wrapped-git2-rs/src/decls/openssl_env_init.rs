macro_rules! openssl_env_init {
    () => {
        # [cfg (any (windows , target_os = "macos" , target_os = "ios" , not (feature = "https")))] fn openssl_env_init () { }
    };
}

openssl_env_init!()