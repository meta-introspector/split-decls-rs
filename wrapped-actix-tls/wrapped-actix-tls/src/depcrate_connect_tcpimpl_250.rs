// Generated macro for impl_250 (impl)
macro_rules! Depcrate_connect_tcpimpl_250 {
() => {
// Module: crate::connect::tcp
// Provides: {"impl_250"}
// Dependencies: {}
impl < R : Host > Service < ConnectInfo < R > > for TcpConnectorService { type Response = Connection < R , TcpStream > ; type Error = ConnectError ; type Future = TcpConnectorFut < R > ; actix_service :: always_ready ! () ; fn call (& self , req : ConnectInfo < R >) -> Self :: Future { let port = req . port () ; let ConnectInfo { request : req , addr , local_addr , .. } = req ; TcpConnectorFut :: new (req , port , local_addr , addr) } }
};
}
