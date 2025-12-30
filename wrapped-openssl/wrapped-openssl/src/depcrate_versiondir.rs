// Generated macro for dir (function)
macro_rules! Depcrate_versiondir {
() => {
// Module: crate::version
// Provides: {"dir"}
// Dependencies: {}
# [doc = " The \"OPENSSLDIR\" setting of the library build in the form \"OPENSSLDIR: \"...\"\" if available or \"OPENSSLDIR: N/A\" otherwise."] # [corresponds (OpenSSL_version)] pub fn dir () -> & 'static str { unsafe { CStr :: from_ptr (OpenSSL_version (OPENSSL_DIR)) . to_str () . unwrap () } }
};
}
