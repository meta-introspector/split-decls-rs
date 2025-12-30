// Generated macro for built_on (function)
macro_rules! Depcrate_versionbuilt_on {
() => {
// Module: crate::version
// Provides: {"built_on"}
// Dependencies: {}
# [doc = " The date of the build process in the form \"built on: ...\" if available or \"built on: date not available\" otherwise."] # [corresponds (OpenSSL_version)] pub fn built_on () -> & 'static str { unsafe { CStr :: from_ptr (OpenSSL_version (OPENSSL_BUILT_ON)) . to_str () . unwrap () } }
};
}
