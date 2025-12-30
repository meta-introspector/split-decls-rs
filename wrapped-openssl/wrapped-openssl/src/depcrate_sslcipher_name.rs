// Generated macro for cipher_name (function)
macro_rules! Depcrate_sslcipher_name {
() => {
// Module: crate::ssl
// Provides: {"cipher_name"}
// Dependencies: {}
# [doc = " Returns the OpenSSL name of a cipher corresponding to an RFC-standard cipher name."] # [doc = ""] # [doc = " If the cipher has no corresponding OpenSSL name, the string `(NONE)` is returned."] # [doc = ""] # [doc = " Requires OpenSSL 1.1.1 or newer."] # [corresponds (OPENSSL_cipher_name)] # [cfg (ossl111)] pub fn cipher_name (std_name : & str) -> & 'static str { unsafe { ffi :: init () ; let s = CString :: new (std_name) . unwrap () ; let ptr = ffi :: OPENSSL_cipher_name (s . as_ptr ()) ; CStr :: from_ptr (ptr) . to_str () . unwrap () } }
};
}
