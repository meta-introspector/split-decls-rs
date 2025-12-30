// Generated macro for test_mozilla_server (function)
macro_rules! Depcrate_ssl_testtest_mozilla_server {
() => {
// Module: crate::ssl::test
// Provides: {"test_mozilla_server"}
// Dependencies: {}
fn test_mozilla_server (new : fn (SslMethod) -> Result < SslAcceptorBuilder , ErrorStack >) { let listener = TcpListener :: bind ("127.0.0.1:0") . unwrap () ; let port = listener . local_addr () . unwrap () . port () ; let t = thread :: spawn (move | | { let key = PKey :: private_key_from_pem (KEY) . unwrap () ; let cert = X509 :: from_pem (CERT) . unwrap () ; let mut acceptor = new (SslMethod :: tls ()) . unwrap () ; acceptor . set_private_key (& key) . unwrap () ; acceptor . set_certificate (& cert) . unwrap () ; let acceptor = acceptor . build () ; let stream = listener . accept () . unwrap () . 0 ; let mut stream = acceptor . accept (stream) . unwrap () ; stream . write_all (b"hello") . unwrap () ; }) ; let mut connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_ca_file ("test/root-ca.pem") . unwrap () ; let connector = connector . build () ; let stream = TcpStream :: connect (("127.0.0.1" , port)) . unwrap () ; let mut stream = connector . connect ("foobar.com" , stream) . unwrap () ; let mut buf = [0 ; 5] ; stream . read_exact (& mut buf) . unwrap () ; assert_eq ! (b"hello" , & buf) ; t . join () . unwrap () ; }
};
}
