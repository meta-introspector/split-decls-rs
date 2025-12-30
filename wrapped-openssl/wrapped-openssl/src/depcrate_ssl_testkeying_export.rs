// Generated macro for keying_export (function)
macro_rules! Depcrate_ssl_testkeying_export {
() => {
// Module: crate::ssl::test
// Provides: {"keying_export"}
// Dependencies: {}
# [test] fn keying_export () { let listener = TcpListener :: bind ("127.0.0.1:0") . unwrap () ; let addr = listener . local_addr () . unwrap () ; let label = "EXPERIMENTAL test" ; let context = b"my context" ; let guard = thread :: spawn (move | | { let stream = listener . accept () . unwrap () . 0 ; let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_certificate_file (Path :: new ("test/cert.pem") , SslFiletype :: PEM) . unwrap () ; ctx . set_private_key_file (Path :: new ("test/key.pem") , SslFiletype :: PEM) . unwrap () ; let ssl = Ssl :: new (& ctx . build ()) . unwrap () ; let mut stream = ssl . accept (stream) . unwrap () ; let mut buf = [0 ; 32] ; stream . ssl () . export_keying_material (& mut buf , label , Some (context)) . unwrap () ; stream . write_all (& [0]) . unwrap () ; buf }) ; let stream = TcpStream :: connect (addr) . unwrap () ; let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; let ssl = Ssl :: new (& ctx . build ()) . unwrap () ; let mut stream = ssl . connect (stream) . unwrap () ; let mut buf = [1 ; 32] ; stream . ssl () . export_keying_material (& mut buf , label , Some (context)) . unwrap () ; stream . read_exact (& mut [0]) . unwrap () ; let buf2 = guard . join () . unwrap () ; assert_eq ! (buf , buf2) ; }
};
}
