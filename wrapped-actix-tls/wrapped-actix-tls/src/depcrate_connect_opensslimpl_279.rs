// Generated macro for impl_279 (impl)
macro_rules! Depcrate_connect_opensslimpl_279 {
() => {
// Module: crate::connect::openssl
// Provides: {"impl_279"}
// Dependencies: {}
impl < R : Host , IO > Future for ConnectFut < R , IO > where R : Host , IO : ActixStream , { type Output = Result < Connection < R , AsyncSslStream < IO > > , io :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . get_mut () ; match ready ! (Pin :: new (this . io . as_mut () . unwrap ()) . poll_connect (cx)) { Ok (_) => { let stream = this . stream . take () . unwrap () ; trace ! ("TLS handshake success: {:?}" , stream . hostname ()) ; Poll :: Ready (Ok (stream . replace_io (this . io . take () . unwrap ()) . 1)) } Err (err) => { trace ! ("TLS handshake error: {:?}" , err) ; Poll :: Ready (Err (io :: Error :: other (format ! ("{err}")))) } } } }
};
}
