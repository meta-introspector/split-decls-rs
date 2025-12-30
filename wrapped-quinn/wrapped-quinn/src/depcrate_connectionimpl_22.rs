// Generated macro for impl_22 (impl)
macro_rules! Depcrate_connectionimpl_22 {
() => {
// Module: crate::connection
// Provides: {"impl_22"}
// Dependencies: {}
impl Future for OpenUni < '_ > { type Output = Result < SendStream , ConnectionError > ; fn poll (self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let (conn , id , is_0rtt) = ready ! (poll_open (ctx , this . conn , this . notify , Dir :: Uni)) ? ; Poll :: Ready (Ok (SendStream :: new (conn , id , is_0rtt))) } }
};
}
