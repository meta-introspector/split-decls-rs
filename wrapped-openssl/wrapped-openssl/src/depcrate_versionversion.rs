// Generated macro for version (function)
macro_rules! Depcrate_versionversion {
() => {
// Module: crate::version
// Provides: {"version"}
// Dependencies: {}
# [doc = " The text variant of the version number and the release date. For example, \"OpenSSL 0.9.5a 1 Apr 2000\"."] # [corresponds (OpenSSL_version)] pub fn version () -> & 'static str { unsafe { CStr :: from_ptr (OpenSSL_version (OPENSSL_VERSION)) . to_str () . unwrap () } }
};
}
