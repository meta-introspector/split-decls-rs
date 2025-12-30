// Generated macro for impl_1216 (impl)
macro_rules! Depcrate_sslimpl_1216 {
() => {
// Module: crate::ssl
// Provides: {"impl_1216"}
// Dependencies: {}
impl ToOwned for SslSessionRef { type Owned = SslSession ; fn to_owned (& self) -> SslSession { unsafe { SSL_SESSION_up_ref (self . as_ptr ()) ; SslSession (self . as_ptr ()) } } }
};
}
