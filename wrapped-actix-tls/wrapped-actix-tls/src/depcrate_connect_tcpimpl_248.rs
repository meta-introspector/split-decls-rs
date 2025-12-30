// Generated macro for impl_248 (impl)
macro_rules! Depcrate_connect_tcpimpl_248 {
() => {
// Module: crate::connect::tcp
// Provides: {"impl_248"}
// Dependencies: {}
impl < R : Host > ServiceFactory < ConnectInfo < R > > for TcpConnector { type Response = Connection < R , TcpStream > ; type Error = ConnectError ; type Config = () ; type Service = TcpConnectorService ; type InitError = () ; type Future = Ready < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : ()) -> Self :: Future { ok (self . service ()) } }
};
}
