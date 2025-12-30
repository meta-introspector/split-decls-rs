// Generated macro for impl_129 (impl)
macro_rules! Depcrate_websocket_futuresimpl_129 {
() => {
// Module: crate::websocket::futures
// Provides: {"impl_129"}
// Dependencies: {}
impl TryFrom < web_sys :: WebSocket > for WebSocket { type Error = JsError ; fn try_from (ws : web_sys :: WebSocket) -> Result < Self , Self :: Error > { Self :: setup (Ok (ws)) } }
};
}
