// Generated macro for impl_252 (impl)
macro_rules! Depcrate_connect_tcpimpl_252 {
() => {
// Module: crate::connect::tcp
// Provides: {"impl_252"}
// Dependencies: {}
impl < R : Host > TcpConnectorFut < R > { pub (crate) fn new (req : R , port : u16 , local_addr : Option < IpAddr > , addr : ConnectAddrs ,) -> TcpConnectorFut < R > { if addr . is_unresolved () { error ! ("TCP connector: unresolved connection address") ; return TcpConnectorFut :: Error (Some (ConnectError :: Unresolved)) ; } trace ! ("TCP connector: connecting to {} on port {}" , req . hostname () , port) ; match addr { ConnectAddrs :: None => unreachable ! ("none variant already checked") , ConnectAddrs :: One (addr) => TcpConnectorFut :: Response { req : Some (req) , port , local_addr , addrs : None , stream : ReusableBoxFuture :: new (connect (addr , local_addr)) , } , ConnectAddrs :: Multi (mut addrs) => { let addr = addrs . pop_front () . unwrap () ; TcpConnectorFut :: Response { req : Some (req) , port , local_addr , addrs : Some (addrs) , stream : ReusableBoxFuture :: new (connect (addr , local_addr)) , } } } } }
};
}
