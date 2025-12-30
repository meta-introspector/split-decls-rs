// Generated macro for server_tls11_only (function)
macro_rules! Depcrate_testserver_tls11_only {
() => {
// Module: crate::test
// Provides: {"server_tls11_only"}
// Dependencies: {}
# [test] fn server_tls11_only () { let keys = test_cert_gen :: keys () ; let identity = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let builder = p ! (TlsAcceptor :: builder (identity) . min_protocol_version (Some (Protocol :: Tlsv12)) . max_protocol_version (Some (Protocol :: Tlsv12)) . build ()) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; let mut socket = p ! (builder . accept (socket)) ; let mut buf = [0 ; 5] ; p ! (socket . read_exact (& mut buf)) ; assert_eq ! (& buf , b"hello") ; p ! (socket . write_all (b"world")) ; }) ; let root_ca = Certificate :: from_der (keys . client . ca . get_der ()) . unwrap () ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let builder = p ! (TlsConnector :: builder () . add_root_certificate (root_ca) . min_protocol_version (Some (Protocol :: Tlsv12)) . max_protocol_version (Some (Protocol :: Tlsv12)) . build ()) ; let mut socket = p ! (builder . connect ("localhost" , socket)) ; p ! (socket . write_all (b"hello")) ; let mut buf = vec ! [] ; p ! (socket . read_to_end (& mut buf)) ; assert_eq ! (buf , b"world") ; p ! (j . join ()) ; }
};
}
