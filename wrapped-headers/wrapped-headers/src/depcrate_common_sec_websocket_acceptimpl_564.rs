// Generated macro for impl_564 (impl)
macro_rules! Depcrate_common_sec_websocket_acceptimpl_564 {
() => {
// Module: crate::common::sec_websocket_accept
// Provides: {"impl_564"}
// Dependencies: {}
impl From < SecWebsocketKey > for SecWebsocketAccept { fn from (key : SecWebsocketKey) -> SecWebsocketAccept { sign (key . 0 . as_bytes ()) } }
};
}
