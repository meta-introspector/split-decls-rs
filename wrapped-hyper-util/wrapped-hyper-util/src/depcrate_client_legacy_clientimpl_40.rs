// Generated macro for impl_40 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_40 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_40"}
// Dependencies: {}
impl Future for ResponseFuture { type Output = Result < Response < hyper :: body :: Incoming > , Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { self . inner . get_mut () . as_mut () . poll (cx) } }
};
}
