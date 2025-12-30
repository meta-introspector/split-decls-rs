// Generated macro for impl_1067 (impl)
macro_rules! Depcrate_ssl_test_serverimpl_1067 {
() => {
// Module: crate::ssl::test::server
// Provides: {"impl_1067"}
// Dependencies: {}
impl Server { pub fn builder () -> Builder { let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_certificate_chain_file ("test/cert.pem") . unwrap () ; ctx . set_private_key_file ("test/key.pem" , SslFiletype :: PEM) . unwrap () ; Builder { ctx , ssl_cb : Box :: new (| _ | { }) , io_cb : Box :: new (| _ | { }) , should_error : false , } } pub fn client (& self) -> ClientBuilder { ClientBuilder { ctx : SslContext :: builder (SslMethod :: tls ()) . unwrap () , addr : self . addr , } } pub fn connect_tcp (& self) -> TcpStream { TcpStream :: connect (self . addr) . unwrap () } }
};
}
