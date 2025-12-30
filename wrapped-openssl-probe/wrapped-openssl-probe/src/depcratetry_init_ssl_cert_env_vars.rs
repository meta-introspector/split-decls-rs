// Generated macro for try_init_ssl_cert_env_vars (function)
macro_rules! Depcratetry_init_ssl_cert_env_vars {
() => {
// Module: crate
// Provides: {"try_init_ssl_cert_env_vars"}
// Dependencies: {}
# [doc = " Deprecated as this isn't sound, use [`try_init_openssl_env_vars`] instead."] # [doc (hidden)] # [deprecated (note = "use try_init_openssl_env_vars instead, this function is not safe")] pub fn try_init_ssl_cert_env_vars () -> bool { unsafe { try_init_openssl_env_vars () } }
};
}
