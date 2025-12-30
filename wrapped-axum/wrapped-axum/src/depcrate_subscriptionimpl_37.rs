// Generated macro for impl_37 (impl)
macro_rules! Depcrate_subscriptionimpl_37 {
() => {
// Module: crate::subscription
// Provides: {"impl_37"}
// Dependencies: {}
impl < S > FromRequestParts < S > for GraphQLProtocol where S : Send + Sync , { type Rejection = StatusCode ; async fn from_request_parts (parts : & mut Parts , _state : & S) -> Result < Self , Self :: Rejection > { parts . headers . get (http :: header :: SEC_WEBSOCKET_PROTOCOL) . and_then (| value | value . to_str () . ok ()) . and_then (| protocols | { protocols . split (',') . find_map (| p | WebSocketProtocols :: from_str (p . trim ()) . ok ()) }) . map (Self) . ok_or (StatusCode :: BAD_REQUEST) } }
};
}
