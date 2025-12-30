// Generated macro for macro_1218 (macro)
macro_rules! Depcrate_sslmacro_1218 {
() => {
// Module: crate::ssl
// Provides: {"macro_1218"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: SSL ; fn drop = ffi :: SSL_free ; # [doc = " The state of an SSL/TLS session."] # [doc = ""] # [doc = " `Ssl` objects are created from an [`SslContext`], which provides configuration defaults."] # [doc = " These defaults can be overridden on a per-`Ssl` basis, however."] # [doc = ""] # [doc = " [`SslContext`]: struct.SslContext.html"] pub struct Ssl ; # [doc = " Reference to an [`Ssl`]."] # [doc = ""] # [doc = " [`Ssl`]: struct.Ssl.html"] pub struct SslRef ; }
};
}
