// Generated macro for server_pkcs8 (function)
macro_rules! Depcrate_testserver_pkcs8 {
() => {
// Module: crate::test
// Provides: {"server_pkcs8"}
// Dependencies: {}
# [test] fn server_pkcs8 () { let keys = test_cert_gen :: keys () ; let cert = keys . server . cert_and_key . cert . to_pem () . into_bytes () ; let key = rsa_to_pkcs8 (& keys . server . cert_and_key . key . to_pem_incorrect ()) . into_bytes () ; let ident = Identity :: from_pkcs8 (& cert , & key) . unwrap () ; let ident2 = ident . clone () ; let builder = p ! (TlsAcceptor :: new (ident)) ; let listener = p ! (TcpListener :: bind ("0.0.0.0:0")) ; let port = p ! (listener . local_addr ()) . port () ; let j = thread :: spawn (move | | { let socket = p ! (listener . accept ()) . 0 ; let mut socket = p ! (builder . accept (socket)) ; let mut buf = [0 ; 5] ; p ! (socket . read_exact (& mut buf)) ; assert_eq ! (& buf , b"hello") ; p ! (socket . write_all (b"world")) ; }) ; let root_ca = Certificate :: from_der (keys . client . ca . get_der ()) . unwrap () ; let socket = p ! (TcpStream :: connect (("localhost" , port))) ; let mut builder = TlsConnector :: builder () ; builder . identity (ident2) ; builder . add_root_certificate (root_ca) ; let builder = p ! (builder . build ()) ; let mut socket = p ! (builder . connect ("localhost" , socket)) ; p ! (socket . write_all (b"hello")) ; let mut buf = vec ! [] ; p ! (socket . read_to_end (& mut buf)) ; assert_eq ! (buf , b"world") ; p ! (j . join ()) ; }
};
}
