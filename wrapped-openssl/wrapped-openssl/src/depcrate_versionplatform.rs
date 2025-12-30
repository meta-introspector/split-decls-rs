// Generated macro for platform (function)
macro_rules! Depcrate_versionplatform {
() => {
// Module: crate::version
// Provides: {"platform"}
// Dependencies: {}
# [doc = " The \"Configure\" target of the library build in the form \"platform: ...\" if available or \"platform: information not available\" otherwise."] # [corresponds (OpenSSL_version)] pub fn platform () -> & 'static str { unsafe { CStr :: from_ptr (OpenSSL_version (OPENSSL_PLATFORM)) . to_str () . unwrap () } }
};
}
