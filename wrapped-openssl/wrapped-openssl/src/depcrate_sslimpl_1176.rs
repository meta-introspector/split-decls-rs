// Generated macro for impl_1176 (impl)
macro_rules! Depcrate_sslimpl_1176 {
() => {
// Module: crate::ssl
// Provides: {"impl_1176"}
// Dependencies: {}
impl NameType { # [doc = " A host name."] pub const HOST_NAME : NameType = NameType (ffi :: TLSEXT_NAMETYPE_host_name) ; # [doc = " Constructs a `StatusType` from a raw OpenSSL value."] pub fn from_raw (raw : c_int) -> StatusType { StatusType (raw) } # [doc = " Returns the raw OpenSSL value represented by this type."] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
