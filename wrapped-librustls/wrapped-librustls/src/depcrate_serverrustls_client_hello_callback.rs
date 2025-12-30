// Generated macro for rustls_client_hello_callback (type)
macro_rules! Depcrate_serverrustls_client_hello_callback {
() => {
// Module: crate::server
// Provides: {"rustls_client_hello_callback"}
// Dependencies: {}
# [doc = " Prototype of a callback that can be installed by the application at the"] # [doc = " `rustls_server_config`."] # [doc = ""] # [doc = " This callback will be invoked by a `rustls_connection` once the TLS client"] # [doc = " hello message has been received."] # [doc = ""] # [doc = " `userdata` will be set based on rustls_connection_set_userdata."] # [doc = ""] # [doc = " `hello` gives the value of the available client announcements, as interpreted"] # [doc = " by rustls. See the definition of `rustls_client_hello` for details."] # [doc = ""] # [doc = " NOTE:"] # [doc = " - the passed in `hello` and all its values are only available during the"] # [doc = "   callback invocations."] # [doc = " - the passed callback function must be safe to call multiple times concurrently"] # [doc = "   with the same userdata, unless there is only a single config and connection"] # [doc = "   where it is installed."] # [doc = ""] # [doc = " EXPERIMENTAL: this feature of rustls-ffi is likely to change in the future, as"] # [doc = " the rustls library is re-evaluating their current approach to client hello handling."] pub type rustls_client_hello_callback = Option < unsafe extern "C" fn (userdata : rustls_client_hello_userdata , hello : * const rustls_client_hello ,) -> * const rustls_certified_key , > ;
};
}
