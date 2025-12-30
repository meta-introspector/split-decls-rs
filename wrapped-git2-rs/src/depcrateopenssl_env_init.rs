// Generated macro for openssl_env_init (function)
macro_rules! Depcrateopenssl_env_init {
() => {
// Module: crate
// Provides: {"openssl_env_init"}
// Dependencies: {}
# [cfg (any (windows , target_os = "macos" , target_os = "ios" , not (feature = "https")))] fn openssl_env_init () { }
};
}
