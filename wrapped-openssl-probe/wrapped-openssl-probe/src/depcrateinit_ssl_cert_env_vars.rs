// Generated macro for init_ssl_cert_env_vars (function)
macro_rules! Depcrateinit_ssl_cert_env_vars {
() => {
// Module: crate
// Provides: {"init_ssl_cert_env_vars"}
// Dependencies: {}
# [doc = " Deprecated as this isn't sound, use [`init_openssl_env_vars`] instead."] # [doc (hidden)] # [deprecated (note = "this function is not safe, use `init_openssl_env_vars` instead")] pub fn init_ssl_cert_env_vars () { unsafe { init_openssl_env_vars () ; } }
};
}
