// Generated macro for impl_37 (impl)
macro_rules! Depcrate_subscriptionimpl_37 {
() => {
// Module: crate::subscription
// Provides: {"impl_37"}
// Dependencies: {}
impl < E > Endpoint for GraphQLSubscription < E > where E : Executor , { type Output = Response ; async fn call (& self , req : Request) -> Result < Self :: Output > { let (req , mut body) = req . split () ; let websocket = WebSocket :: from_request (& req , & mut body) . await ? ; let protocol = GraphQLProtocol :: from_request (& req , & mut body) . await ? ; let executor = self . executor . clone () ; let resp = websocket . protocols (ALL_WEBSOCKET_PROTOCOLS) . on_upgrade (move | stream | GraphQLWebSocket :: new (stream , executor , protocol) . serve ()) . into_response () ; Ok (resp) } }
};
}
