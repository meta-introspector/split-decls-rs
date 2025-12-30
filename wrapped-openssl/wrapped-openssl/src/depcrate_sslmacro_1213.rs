// Generated macro for macro_1213 (macro)
macro_rules! Depcrate_sslmacro_1213 {
() => {
// Module: crate::ssl
// Provides: {"macro_1213"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: SSL_SESSION ; fn drop = ffi :: SSL_SESSION_free ; # [doc = " An encoded SSL session."] # [doc = ""] # [doc = " These can be cached to share sessions across connections."] pub struct SslSession ; # [doc = " Reference to [`SslSession`]."] # [doc = ""] # [doc = " [`SslSession`]: struct.SslSession.html"] pub struct SslSessionRef ; }
};
}
