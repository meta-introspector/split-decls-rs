// Generated macro for impl_191 (impl)
macro_rules! Depcrate_connect_connectorimpl_191 {
() => {
// Module: crate::connect::connector
// Provides: {"impl_191"}
// Dependencies: {}
impl < R : Host > Service < ConnectInfo < R > > for ConnectorService { type Response = Connection < R , TcpStream > ; type Error = ConnectError ; type Future = ConnectServiceResponse < R > ; actix_service :: always_ready ! () ; fn call (& self , req : ConnectInfo < R >) -> Self :: Future { ConnectServiceResponse { fut : ConnectFut :: Resolve (self . resolver . call (req)) , tcp : self . tcp , } } }
};
}
