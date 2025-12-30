// Generated macro for impl_572 (impl)
macro_rules! Depcrate_common_sec_websocket_keyimpl_572 {
() => {
// Module: crate::common::sec_websocket_key
// Provides: {"impl_572"}
// Dependencies: {}
impl From < [u8 ; 16] > for SecWebsocketKey { fn from (bytes : [u8 ; 16]) -> Self { let mut value = HeaderValue :: from_str (& STANDARD . encode (bytes)) . unwrap () ; value . set_sensitive (true) ; Self (value) } }
};
}
