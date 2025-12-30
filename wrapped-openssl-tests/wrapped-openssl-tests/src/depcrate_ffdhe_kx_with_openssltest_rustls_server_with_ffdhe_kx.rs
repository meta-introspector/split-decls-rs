// Generated macro for test_rustls_server_with_ffdhe_kx (function)
macro_rules! Depcrate_ffdhe_kx_with_openssltest_rustls_server_with_ffdhe_kx {
() => {
// Module: crate::ffdhe_kx_with_openssl
// Provides: {"test_rustls_server_with_ffdhe_kx"}
// Dependencies: {}
fn test_rustls_server_with_ffdhe_kx (provider : CryptoProvider , iters : usize) { verify_openssl3_available () ; let message = "Hello from rustls!\n" ; let listener = TcpListener :: bind (("localhost" , 0)) . unwrap () ; let port = listener . local_addr () . unwrap () . port () ; let server_thread = thread :: spawn (move | | { let config = Arc :: new (server_config_with_ffdhe_kx (provider)) ; for _ in 0 .. iters { let mut server = rustls :: ServerConnection :: new (config . clone ()) . unwrap () ; let (mut tcp_stream , _addr) = listener . accept () . unwrap () ; server . writer () . write_all (message . as_bytes ()) . unwrap () ; server . complete_io (& mut tcp_stream) . unwrap () ; tcp_stream . flush () . unwrap () ; } }) ; let mut connector = openssl :: ssl :: SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_ca_file (CA_PEM_FILE) . unwrap () ; connector . set_groups_list ("ffdhe2048") . unwrap () ; let connector = connector . build () ; for _iter in 0 .. iters { let stream = TcpStream :: connect (("localhost" , port)) . unwrap () ; let mut stream = connector . connect ("testserver.com" , stream) . unwrap () ; let mut buf = String :: new () ; stream . read_to_string (& mut buf) . unwrap () ; assert_eq ! (buf , message) ; } server_thread . join () . unwrap () ; }
};
}
