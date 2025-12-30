// Generated macro for peer_certificate (function)
macro_rules! Depcrate_testpeer_certificate {
() => {
// Module: crate::test
// Provides: {"peer_certificate"}
// Dependencies: {}
# [test] fn peer_certificate () { let keys = test_cert_gen :: keys () ; let identity = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let builder = p ! (TlsAcceptor :: new (identity)) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; let socket = p ! (builder . accept (socket)) ; assert ! (socket . peer_certificate () . unwrap () . is_none ()) ; }) ; let root_ca = Certificate :: from_der (keys . client . ca . get_der ()) . unwrap () ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let builder = p ! (TlsConnector :: builder () . add_root_certificate (root_ca) . build ()) ; let socket = p ! (builder . connect ("localhost" , socket)) ; let cert = socket . peer_certificate () . unwrap () . unwrap () ; assert_eq ! (cert . to_der () . unwrap () , keys . server . cert_and_key . cert . get_der ()) ; p ! (j . join ()) ; }
};
}
