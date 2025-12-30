// Generated macro for impl_44 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_44 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_44"}
// Dependencies: {}
impl < B : Body + 'static > PoolClient < B > { fn try_send_request (& mut self , req : Request < B > ,) -> impl Future < Output = Result < Response < hyper :: body :: Incoming > , ConnTrySendError < Request < B > > > > where B : Send , { # [cfg (all (feature = "http1" , feature = "http2"))] return match self . tx { # [cfg (feature = "http1")] PoolTx :: Http1 (ref mut tx) => Either :: Left (tx . try_send_request (req)) , # [cfg (feature = "http2")] PoolTx :: Http2 (ref mut tx) => Either :: Right (tx . try_send_request (req)) , } ; # [cfg (feature = "http1")] # [cfg (not (feature = "http2"))] return match self . tx { # [cfg (feature = "http1")] PoolTx :: Http1 (ref mut tx) => tx . try_send_request (req) , } ; # [cfg (not (feature = "http1"))] # [cfg (feature = "http2")] return match self . tx { # [cfg (feature = "http2")] PoolTx :: Http2 (ref mut tx) => tx . try_send_request (req) , } ; } }
};
}
