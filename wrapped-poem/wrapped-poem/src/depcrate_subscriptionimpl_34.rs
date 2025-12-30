// Generated macro for impl_34 (impl)
macro_rules! Depcrate_subscriptionimpl_34 {
() => {
// Module: crate::subscription
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a > FromRequest < 'a > for GraphQLProtocol { async fn from_request (req : & 'a Request , _body : & mut RequestBody) -> Result < Self > { req . headers () . get (http :: header :: SEC_WEBSOCKET_PROTOCOL) . and_then (| value | value . to_str () . ok ()) . and_then (| protocols | { protocols . split (',') . find_map (| p | WebSocketProtocols :: from_str (p . trim ()) . ok ()) }) . map (Self) . ok_or_else (| | Error :: from_status (StatusCode :: BAD_REQUEST)) } }
};
}
