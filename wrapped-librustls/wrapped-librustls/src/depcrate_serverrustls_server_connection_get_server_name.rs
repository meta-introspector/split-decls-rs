// Generated macro for rustls_server_connection_get_server_name (function)
macro_rules! Depcrate_serverrustls_server_connection_get_server_name {
() => {
// Module: crate::server
// Provides: {"rustls_server_connection_get_server_name"}
// Dependencies: {}
# [doc = " Returns a `rustls_str` reference to the server name sent by the client in a server name"] # [doc = " indication (SNI) extension."] # [doc = ""] # [doc = " The returned `rustls_str` is valid until the next mutating function call affecting the"] # [doc = " connection. A mutating function call is one where the first argument has type"] # [doc = " `struct rustls_connection *` (as opposed to `const struct rustls_connection *`). The caller"] # [doc = " does not need to free the `rustls_str`."] # [doc = ""] # [doc = " Returns a zero-length `rustls_str` if:"] # [doc = ""] # [doc = " - the connection is not a server connection."] # [doc = " - the connection is a server connection but the SNI extension in the client hello has not"] # [doc = "   been processed during the handshake yet. Check `rustls_connection_is_handshaking`."] # [doc = " - the SNI value contains null bytes."] # [no_mangle] pub extern "C" fn rustls_server_connection_get_server_name (conn : * const rustls_connection ,) -> rustls_str < 'static > { ffi_panic_boundary ! { let Some (server_connection) = try_ref_from_ptr ! (conn) . as_server () else { return rustls_str :: default () ; } ; let Some (sni_hostname) = server_connection . server_name () else { return rustls_str :: default () ; } ; let res = rustls_str :: try_from (sni_hostname) . unwrap_or_default () ; unsafe { res . into_static () } } }
};
}
