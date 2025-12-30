// Generated macro for impl_29 (impl)
macro_rules! Depcrate_connectionimpl_29 {
() => {
// Module: crate::connection
// Provides: {"impl_29"}
// Dependencies: {}
impl Future for AcceptBi < '_ > { type Output = Result < (SendStream , RecvStream) , ConnectionError > ; fn poll (self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let (conn , id , is_0rtt) = ready ! (poll_accept (ctx , this . conn , this . notify , Dir :: Bi)) ? ; Poll :: Ready (Ok ((SendStream :: new (conn . clone () , id , is_0rtt) , RecvStream :: new (conn , id , is_0rtt) ,))) } }
};
}
