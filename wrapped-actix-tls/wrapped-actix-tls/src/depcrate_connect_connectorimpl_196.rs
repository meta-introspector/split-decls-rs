// Generated macro for impl_196 (impl)
macro_rules! Depcrate_connect_connectorimpl_196 {
() => {
// Module: crate::connect::connector
// Provides: {"impl_196"}
// Dependencies: {}
impl < R : Host > Future for ConnectServiceResponse < R > { type Output = Result < Connection < R , TcpStream > , ConnectError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . fut . poll_connect (cx)) ? { ConnectFutState :: Resolved (res) => { self . fut = ConnectFut :: Connect (self . tcp . call (res)) ; } ConnectFutState :: Connected (res) => return Poll :: Ready (Ok (res)) , } } } }
};
}
