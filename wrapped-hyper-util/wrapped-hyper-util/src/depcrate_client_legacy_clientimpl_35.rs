// Generated macro for impl_35 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_35 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_35"}
// Dependencies: {}
impl < C , B > tower_service :: Service < Request < B > > for & '_ Client < C , B > where C : Connect + Clone + Send + Sync + 'static , B : Body + Send + 'static + Unpin , B :: Data : Send , B :: Error : Into < Box < dyn StdError + Send + Sync > > , { type Response = Response < hyper :: body :: Incoming > ; type Error = Error ; type Future = ResponseFuture ; fn poll_ready (& mut self , _ : & mut task :: Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn call (& mut self , req : Request < B >) -> Self :: Future { self . request (req) } }
};
}
