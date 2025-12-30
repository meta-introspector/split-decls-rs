// Generated macro for impl_815 (impl)
macro_rules! Depcrate_clientimpl_815 {
() => {
// Module: crate::client
// Provides: {"impl_815"}
// Dependencies: {}
impl Future for PushedResponseFuture { type Output = Result < Response < RecvStream > , crate :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . inner) . poll (cx) } }
};
}
