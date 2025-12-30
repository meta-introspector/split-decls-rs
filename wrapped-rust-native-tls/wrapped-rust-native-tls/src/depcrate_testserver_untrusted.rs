// Generated macro for server_untrusted (function)
macro_rules! Depcrate_testserver_untrusted {
() => {
// Module: crate::test
// Provides: {"server_untrusted"}
// Dependencies: {}
# [test] fn server_untrusted () { let keys = test_cert_gen :: keys () ; let identity = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let builder = p ! (TlsAcceptor :: new (identity)) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; let _ = builder . accept (socket) ; }) ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let builder = p ! (TlsConnector :: new ()) ; builder . connect ("localhost" , socket) . unwrap_err () ; p ! (j . join ()) ; }
};
}
