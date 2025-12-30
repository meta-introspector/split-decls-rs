// Generated macro for impl_1174 (impl)
macro_rules! Depcrate_sslimpl_1174 {
() => {
// Module: crate::ssl
// Provides: {"impl_1174"}
// Dependencies: {}
impl StatusType { # [doc = " An OSCP status."] pub const OCSP : StatusType = StatusType (ffi :: TLSEXT_STATUSTYPE_ocsp) ; # [doc = " Constructs a `StatusType` from a raw OpenSSL value."] pub fn from_raw (raw : c_int) -> StatusType { StatusType (raw) } # [doc = " Returns the raw OpenSSL value represented by this type."] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
