// Generated macro for rustls_session_store_put_callback (type)
macro_rules! Depcrate_sessionrustls_session_store_put_callback {
() => {
// Module: crate::session
// Provides: {"rustls_session_store_put_callback"}
// Dependencies: {}
# [doc = " Prototype of a callback that can be installed by the application at the"] # [doc = " `rustls_server_config` or `rustls_client_config`."] # [doc = ""] # [doc = " This callback will be invoked by a TLS session when a TLS session"] # [doc = " been created and an id for later use is handed to the client/has"] # [doc = " been received from the server."] # [doc = ""] # [doc = " `userdata` will be supplied based on rustls_{client,server}_session_set_userdata."] # [doc = ""] # [doc = " The callback should return RUSTLS_RESULT_OK to indicate that a value was"] # [doc = " successfully stored, or RUSTLS_RESULT_IO on failure."] # [doc = ""] # [doc = " NOTE: the passed in `key` and `val` are only available during the"] # [doc = " callback invocation."] # [doc = " NOTE: callbacks used in several sessions via a common config"] # [doc = " must be implemented thread-safe."] pub type rustls_session_store_put_callback = Option < unsafe extern "C" fn (userdata : rustls_session_store_userdata , key : * const rustls_slice_bytes , val : * const rustls_slice_bytes ,) -> u32 , > ;
};
}
