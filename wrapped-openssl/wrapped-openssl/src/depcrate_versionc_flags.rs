// Generated macro for c_flags (function)
macro_rules! Depcrate_versionc_flags {
() => {
// Module: crate::version
// Provides: {"c_flags"}
// Dependencies: {}
# [doc = " The compiler flags set for the compilation process in the form \"compiler: ...\" if available or"] # [doc = " \"compiler: information not available\" otherwise."] # [corresponds (OpenSSL_version)] pub fn c_flags () -> & 'static str { unsafe { CStr :: from_ptr (OpenSSL_version (OPENSSL_CFLAGS)) . to_str () . unwrap () } }
};
}
