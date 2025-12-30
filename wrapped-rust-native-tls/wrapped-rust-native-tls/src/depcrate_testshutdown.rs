// Generated macro for shutdown (function)
macro_rules! Depcrate_testshutdown {
() => {
// Module: crate::test
// Provides: {"shutdown"}
// Dependencies: {}
# [test] fn shutdown () { let keys = test_cert_gen :: keys () ; let identity = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let builder = p ! (TlsAcceptor :: new (identity)) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; let mut socket = p ! (builder . accept (socket)) ; let mut buf = [0 ; 5] ; p ! (socket . read_exact (& mut buf)) ; assert_eq ! (& buf , b"hello") ; assert_eq ! (p ! (socket . read (& mut buf)) , 0) ; p ! (socket . shutdown ()) ; }) ; let root_ca = Certificate :: from_der (keys . client . ca . get_der ()) . unwrap () ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let builder = p ! (TlsConnector :: builder () . add_root_certificate (root_ca) . build ()) ; let mut socket = p ! (builder . connect ("localhost" , socket)) ; p ! (socket . write_all (b"hello")) ; p ! (socket . shutdown ()) ; p ! (j . join ()) ; }
};
}
