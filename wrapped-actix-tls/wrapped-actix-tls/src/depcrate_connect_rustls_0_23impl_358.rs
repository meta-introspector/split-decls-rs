// Generated macro for impl_358 (impl)
macro_rules! Depcrate_connect_rustls_0_23impl_358 {
() => {
// Module: crate::connect::rustls_0_23
// Provides: {"impl_358"}
// Dependencies: {}
impl < R , IO > Future for ConnectFut < R , IO > where R : Host , IO : ActixStream , { type Output = io :: Result < Connection < R , AsyncTlsStream < IO > > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . get_mut () { Self :: InvalidServerName => Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "connection parameters specified invalid server name" ,))) , Self :: Future { connect , connection , } => { let stream = ready ! (Pin :: new (connect) . poll (cx)) ? ; let connection = connection . take () . unwrap () ; tracing :: trace ! ("TLS handshake success: {:?}" , connection . hostname ()) ; Poll :: Ready (Ok (connection . replace_io (stream) . 1)) } } } }
};
}
