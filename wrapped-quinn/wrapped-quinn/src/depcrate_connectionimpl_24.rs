// Generated macro for impl_24 (impl)
macro_rules! Depcrate_connectionimpl_24 {
() => {
// Module: crate::connection
// Provides: {"impl_24"}
// Dependencies: {}
impl Future for OpenBi < '_ > { type Output = Result < (SendStream , RecvStream) , ConnectionError > ; fn poll (self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let (conn , id , is_0rtt) = ready ! (poll_open (ctx , this . conn , this . notify , Dir :: Bi)) ? ; Poll :: Ready (Ok ((SendStream :: new (conn . clone () , id , is_0rtt) , RecvStream :: new (conn , id , is_0rtt) ,))) } }
};
}
