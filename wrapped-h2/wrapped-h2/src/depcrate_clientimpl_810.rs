// Generated macro for impl_810 (impl)
macro_rules! Depcrate_clientimpl_810 {
() => {
// Module: crate::client
// Provides: {"impl_810"}
// Dependencies: {}
impl Future for ResponseFuture { type Output = Result < Response < RecvStream > , crate :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let (parts , _) = ready ! (self . inner . poll_response (cx)) ? . into_parts () ; let body = RecvStream :: new (FlowControl :: new (self . inner . clone ())) ; Poll :: Ready (Ok (Response :: from_parts (parts , body))) } }
};
}
