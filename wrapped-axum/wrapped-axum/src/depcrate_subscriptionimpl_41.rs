// Generated macro for impl_41 (impl)
macro_rules! Depcrate_subscriptionimpl_41 {
() => {
// Module: crate::subscription
// Provides: {"impl_41"}
// Dependencies: {}
impl < B , E > Service < Request < B > > for GraphQLSubscription < E > where B : HttpBody + Send + 'static , E : Executor , { type Response = Response < Body > ; type Error = Infallible ; type Future = BoxFuture < 'static , Result < Self :: Response , Self :: Error > > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn call (& mut self , req : Request < B >) -> Self :: Future { let executor = self . executor . clone () ; Box :: pin (async move { let (mut parts , _body) = req . into_parts () ; let protocol = match GraphQLProtocol :: from_request_parts (& mut parts , & ()) . await { Ok (protocol) => protocol , Err (err) => return Ok (err . into_response ()) , } ; let upgrade = match WebSocketUpgrade :: from_request_parts (& mut parts , & ()) . await { Ok (protocol) => protocol , Err (err) => return Ok (err . into_response ()) , } ; let executor = executor . clone () ; let resp = upgrade . protocols (ALL_WEBSOCKET_PROTOCOLS) . on_upgrade (move | stream | { GraphQLWebSocket :: new (stream , executor , protocol) . serve () }) ; Ok (resp . into_response ()) }) } }
};
}
