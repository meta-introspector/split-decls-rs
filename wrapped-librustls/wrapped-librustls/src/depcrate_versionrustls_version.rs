// Generated macro for rustls_version (function)
macro_rules! Depcrate_versionrustls_version {
() => {
// Module: crate::version
// Provides: {"rustls_version"}
// Dependencies: {}
# [doc = " Returns a static string containing the rustls-ffi version as well as the"] # [doc = " rustls version. The string is alive for the lifetime of the program and does"] # [doc = " not need to be freed."] # [no_mangle] pub extern "C" fn rustls_version () -> rustls_str < 'static > { rustls_str :: from_str_unchecked (RUSTLS_FFI_VERSION) }
};
}
