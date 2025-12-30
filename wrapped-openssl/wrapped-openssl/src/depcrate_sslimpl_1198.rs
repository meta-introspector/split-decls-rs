// Generated macro for impl_1198 (impl)
macro_rules! Depcrate_sslimpl_1198 {
() => {
// Module: crate::ssl
// Provides: {"impl_1198"}
// Dependencies: {}
impl ToOwned for SslContextRef { type Owned = SslContext ; fn to_owned (& self) -> Self :: Owned { unsafe { SSL_CTX_up_ref (self . as_ptr ()) ; SslContext :: from_ptr (self . as_ptr ()) } } }
};
}
