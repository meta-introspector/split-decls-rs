// Generated macro for SessionStoreGetCallback (type)
macro_rules! Depcrate_sessionSessionStoreGetCallback {
() => {
// Module: crate::session
// Provides: {"SessionStoreGetCallback"}
// Dependencies: {}
pub (crate) type SessionStoreGetCallback = unsafe extern "C" fn (userdata : rustls_session_store_userdata , key : * const rustls_slice_bytes , remove_after : c_int , buf : * mut u8 , count : size_t , out_n : * mut size_t ,) -> u32 ;
};
}
