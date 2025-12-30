// Generated macro for impl_27 (impl)
macro_rules! Depcrate_connectionimpl_27 {
() => {
// Module: crate::connection
// Provides: {"impl_27"}
// Dependencies: {}
impl Future for AcceptUni < '_ > { type Output = Result < RecvStream , ConnectionError > ; fn poll (self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let (conn , id , is_0rtt) = ready ! (poll_accept (ctx , this . conn , this . notify , Dir :: Uni)) ? ; Poll :: Ready (Ok (RecvStream :: new (conn , id , is_0rtt))) } }
};
}
