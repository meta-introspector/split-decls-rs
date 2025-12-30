// Generated macro for impl_1172 (impl)
macro_rules! Depcrate_sslimpl_1172 {
() => {
// Module: crate::ssl
// Provides: {"impl_1172"}
// Dependencies: {}
impl SslFiletype { # [doc = " The PEM format."] # [doc = ""] # [doc = " This corresponds to `SSL_FILETYPE_PEM`."] pub const PEM : SslFiletype = SslFiletype (ffi :: SSL_FILETYPE_PEM) ; # [doc = " The ASN1 format."] # [doc = ""] # [doc = " This corresponds to `SSL_FILETYPE_ASN1`."] pub const ASN1 : SslFiletype = SslFiletype (ffi :: SSL_FILETYPE_ASN1) ; # [doc = " Constructs an `SslFiletype` from a raw OpenSSL value."] pub fn from_raw (raw : c_int) -> SslFiletype { SslFiletype (raw) } # [doc = " Returns the raw OpenSSL value represented by this type."] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
