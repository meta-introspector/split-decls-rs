// Generated macro for server_untrusted_unverified (function)
macro_rules! Depcrate_testserver_untrusted_unverified {
() => {
// Module: crate::test
// Provides: {"server_untrusted_unverified"}
// Dependencies: {}
# [test] fn server_untrusted_unverified () { let keys = test_cert_gen :: keys () ; let identity = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let builder = p ! (TlsAcceptor :: new (identity)) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; let mut socket = p ! (builder . accept (socket)) ; let mut buf = [0 ; 5] ; p ! (socket . read_exact (& mut buf)) ; assert_eq ! (& buf , b"hello") ; p ! (socket . write_all (b"world")) ; }) ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let builder = p ! (TlsConnector :: builder () . danger_accept_invalid_certs (true) . build ()) ; let mut socket = p ! (builder . connect ("localhost" , socket)) ; p ! (socket . write_all (b"hello")) ; let mut buf = vec ! [] ; p ! (socket . read_to_end (& mut buf)) ; assert_eq ! (buf , b"world") ; p ! (j . join ()) ; }
};
}
