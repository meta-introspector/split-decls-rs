// Generated macro for SessionStorePutCallback (type)
macro_rules! Depcrate_sessionSessionStorePutCallback {
() => {
// Module: crate::session
// Provides: {"SessionStorePutCallback"}
// Dependencies: {}
pub (crate) type SessionStorePutCallback = unsafe extern "C" fn (userdata : rustls_session_store_userdata , key : * const rustls_slice_bytes , val : * const rustls_slice_bytes ,) -> u32 ;
};
}
