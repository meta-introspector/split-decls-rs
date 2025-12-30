// Generated macro for impl_298 (impl)
macro_rules! Depcrate_connect_rustls_0_20impl_298 {
() => {
// Module: crate::connect::rustls_0_20
// Provides: {"impl_298"}
// Dependencies: {}
impl < R , IO > Future for ConnectFut < R , IO > where R : Host , IO : ActixStream , { type Output = io :: Result < Connection < R , AsyncTlsStream < IO > > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . get_mut () { Self :: InvalidDns => Poll :: Ready (Err (io :: Error :: other ("Rustls v0.20 can only handle hostname-based connections. Enable the `rustls-0_21` \
                feature and use the Rustls v0.21 utilities to gain this feature." ,))) , Self :: Future { connect , connection , } => { let stream = ready ! (Pin :: new (connect) . poll (cx)) ? ; let connection = connection . take () . unwrap () ; tracing :: trace ! ("TLS handshake success: {:?}" , connection . hostname ()) ; Poll :: Ready (Ok (connection . replace_io (stream) . 1)) } } } }
};
}
