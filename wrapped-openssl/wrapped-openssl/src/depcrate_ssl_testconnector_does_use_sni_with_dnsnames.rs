// Generated macro for connector_does_use_sni_with_dnsnames (function)
macro_rules! Depcrate_ssl_testconnector_does_use_sni_with_dnsnames {
() => {
// Module: crate::ssl::test
// Provides: {"connector_does_use_sni_with_dnsnames"}
// Dependencies: {}
# [test] fn connector_does_use_sni_with_dnsnames () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut builder = Server :: builder () ; builder . ctx () . set_servername_callback (| ssl , _ | { assert_eq ! (ssl . servername (NameType :: HOST_NAME) , Some ("foobar.com")) ; CALLED_BACK . store (true , Ordering :: SeqCst) ; Ok (()) }) ; let server = builder . build () ; let mut connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_ca_file ("test/root-ca.pem") . unwrap () ; let s = server . connect_tcp () ; let mut s = connector . build () . configure () . unwrap () . connect ("foobar.com" , s) . unwrap () ; s . read_exact (& mut [0]) . unwrap () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
