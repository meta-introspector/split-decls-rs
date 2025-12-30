// Generated macro for server_no_shared_protocol (function)
macro_rules! Depcrate_testserver_no_shared_protocol {
() => {
// Module: crate::test
// Provides: {"server_no_shared_protocol"}
// Dependencies: {}
# [test] fn server_no_shared_protocol () { let keys = test_cert_gen :: keys () ; let identity = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let builder = p ! (TlsAcceptor :: builder (identity) . min_protocol_version (Some (Protocol :: Tlsv12)) . build ()) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; assert ! (builder . accept (socket) . is_err ()) ; }) ; let root_ca = Certificate :: from_der (keys . client . ca . get_der ()) . unwrap () ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let builder = p ! (TlsConnector :: builder () . add_root_certificate (root_ca) . min_protocol_version (Some (Protocol :: Tlsv11)) . max_protocol_version (Some (Protocol :: Tlsv11)) . build ()) ; assert ! (builder . connect ("localhost" , socket) . is_err ()) ; p ! (j . join ()) ; }
};
}
