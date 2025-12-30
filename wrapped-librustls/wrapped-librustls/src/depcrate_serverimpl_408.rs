// Generated macro for impl_408 (impl)
macro_rules! Depcrate_serverimpl_408 {
() => {
// Module: crate::server
// Provides: {"impl_408"}
// Dependencies: {}
impl rustls_server_config_builder { # [doc = " Register a callback to be invoked when a connection created from this config"] # [doc = " sees a TLS ClientHello message. If `userdata` has been set with"] # [doc = " rustls_connection_set_userdata, it will be passed to the callback."] # [doc = " Otherwise the userdata param passed to the callback will be NULL."] # [doc = ""] # [doc = " Any existing `ResolvesServerCert` implementation currently installed in the"] # [doc = " `rustls_server_config` will be replaced. This also means registering twice"] # [doc = " will overwrite the first registration. It is not permitted to pass a NULL"] # [doc = " value for `callback`."] # [doc = ""] # [doc = " EXPERIMENTAL: this feature of rustls-ffi is likely to change in the future, as"] # [doc = " the rustls library is re-evaluating their current approach to client hello handling."] # [doc = " Installing a client_hello callback will replace any configured certified keys"] # [doc = " and vice versa. Same holds true for the set_certified_keys variant."] # [no_mangle] pub extern "C" fn rustls_server_config_builder_set_hello_callback (builder : * mut rustls_server_config_builder , callback : rustls_client_hello_callback ,) -> rustls_result { ffi_panic_boundary ! { let callback = match callback { Some (cb) => cb , None => return rustls_result :: NullParameter , } ; let builder = try_mut_from_ptr ! (builder) ; builder . cert_resolver = Some (Arc :: new (ClientHelloResolver :: new (callback))) ; rustls_result :: Ok } } }
};
}
