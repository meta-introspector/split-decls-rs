// Generated macro for TcpConnectorFut (enum)
macro_rules! Depcrate_connect_tcpTcpConnectorFut {
() => {
// Module: crate::connect::tcp
// Provides: {"TcpConnectorFut"}
// Dependencies: {}
# [doc = " Connect future for TCP service."] # [doc (hidden)] pub enum TcpConnectorFut < R > { Response { req : Option < R > , port : u16 , local_addr : Option < IpAddr > , addrs : Option < VecDeque < SocketAddr > > , stream : ReusableBoxFuture < 'static , Result < TcpStream , io :: Error > > , } , Error (Option < ConnectError >) , }
};
}
