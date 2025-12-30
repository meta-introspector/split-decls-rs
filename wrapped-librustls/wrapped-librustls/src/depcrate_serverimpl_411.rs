// Generated macro for impl_411 (impl)
macro_rules! Depcrate_serverimpl_411 {
() => {
// Module: crate::server
// Provides: {"impl_411"}
// Dependencies: {}
impl rustls_server_config_builder { # [doc = " Register callbacks for persistence of TLS session IDs and secrets. Both"] # [doc = " keys and values are highly sensitive data, containing enough information"] # [doc = " to break the security of the connections involved."] # [doc = ""] # [doc = " If `builder`, `get_cb`, or `put_cb` are NULL, this function will return"] # [doc = " immediately without doing anything."] # [doc = ""] # [doc = " If `userdata` has been set with rustls_connection_set_userdata, it"] # [doc = " will be passed to the callbacks. Otherwise the userdata param passed to"] # [doc = " the callbacks will be NULL."] # [no_mangle] pub extern "C" fn rustls_server_config_builder_set_persistence (builder : * mut rustls_server_config_builder , get_cb : rustls_session_store_get_callback , put_cb : rustls_session_store_put_callback ,) { ffi_panic_boundary ! { let Some (get_cb) = get_cb else { return ; } ; let Some (put_cb) = put_cb else { return ; } ; try_mut_from_ptr ! (builder) . session_storage = Some (Arc :: new (SessionStoreBroker :: new (get_cb , put_cb))) ; } } }
};
}
