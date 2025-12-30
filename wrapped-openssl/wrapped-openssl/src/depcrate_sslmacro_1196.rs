// Generated macro for macro_1196 (macro)
macro_rules! Depcrate_sslmacro_1196 {
() => {
// Module: crate::ssl
// Provides: {"macro_1196"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: SSL_CTX ; fn drop = ffi :: SSL_CTX_free ; # [doc = " A context object for TLS streams."] # [doc = ""] # [doc = " Applications commonly configure a single `SslContext` that is shared by all of its"] # [doc = " `SslStreams`."] pub struct SslContext ; # [doc = " Reference to [`SslContext`]"] # [doc = ""] # [doc = " [`SslContext`]: struct.SslContext.html"] pub struct SslContextRef ; }
};
}
