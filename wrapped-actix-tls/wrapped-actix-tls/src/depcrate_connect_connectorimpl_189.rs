// Generated macro for impl_189 (impl)
macro_rules! Depcrate_connect_connectorimpl_189 {
() => {
// Module: crate::connect::connector
// Provides: {"impl_189"}
// Dependencies: {}
impl < R : Host > ServiceFactory < ConnectInfo < R > > for Connector { type Response = Connection < R , TcpStream > ; type Error = ConnectError ; type Config = () ; type Service = ConnectorService ; type InitError = () ; type Future = Ready < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : ()) -> Self :: Future { ok (self . service ()) } }
};
}
