// Generated macro for server_no_root_certs (function)
macro_rules! Depcrate_testserver_no_root_certs {
() => {
// Module: crate::test
// Provides: {"server_no_root_certs"}
// Dependencies: {}
# [test] fn server_no_root_certs () { let keys = test_cert_gen :: keys () ; let identity = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let builder = p ! (TlsAcceptor :: new (identity)) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; let mut socket = p ! (builder . accept (socket)) ; let mut buf = [0 ; 5] ; p ! (socket . read_exact (& mut buf)) ; assert_eq ! (& buf , b"hello") ; p ! (socket . write_all (b"world")) ; }) ; let root_ca = Certificate :: from_der (keys . client . ca . get_der ()) . unwrap () ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let builder = p ! (TlsConnector :: builder () . disable_built_in_roots (true) . add_root_certificate (root_ca) . build ()) ; let mut socket = p ! (builder . connect ("localhost" , socket)) ; p ! (socket . write_all (b"hello")) ; let mut buf = vec ! [] ; p ! (socket . read_to_end (& mut buf)) ; assert_eq ! (buf , b"world") ; p ! (j . join ()) ; }
};
}
