// Generated macro for impl_253 (impl)
macro_rules! Depcrate_connect_tcpimpl_253 {
() => {
// Module: crate::connect::tcp
// Provides: {"impl_253"}
// Dependencies: {}
impl < R : Host > Future for TcpConnectorFut < R > { type Output = Result < Connection < R , TcpStream > , ConnectError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . get_mut () { TcpConnectorFut :: Error (err) => Poll :: Ready (Err (err . take () . unwrap ())) , TcpConnectorFut :: Response { req , port , local_addr , addrs , stream , } => loop { match ready ! (stream . poll (cx)) { Ok (sock) => { let req = req . take () . unwrap () ; trace ! ("TCP connector: successfully connected to {:?} - {:?}" , req . hostname () , sock . peer_addr ()) ; return Poll :: Ready (Ok (Connection :: new (req , sock))) ; } Err (err) => { trace ! ("TCP connector: failed to connect to {:?} port: {}" , req . as_ref () . unwrap () . hostname () , port ,) ; if let Some (addr) = addrs . as_mut () . and_then (| addrs | addrs . pop_front ()) { stream . set (connect (addr , * local_addr)) ; } else { return Poll :: Ready (Err (ConnectError :: Io (err))) ; } } } } , } } }
};
}
