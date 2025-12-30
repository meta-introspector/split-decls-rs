// Generated macro for connector_doesnt_use_sni_with_ips (function)
macro_rules! Depcrate_ssl_testconnector_doesnt_use_sni_with_ips {
() => {
// Module: crate::ssl::test
// Provides: {"connector_doesnt_use_sni_with_ips"}
// Dependencies: {}
# [test] fn connector_doesnt_use_sni_with_ips () { static CALLED_BACK : AtomicBool = AtomicBool :: new (false) ; let mut builder = Server :: builder () ; builder . ctx () . set_servername_callback (| ssl , _ | { assert_eq ! (ssl . servername (NameType :: HOST_NAME) , None) ; CALLED_BACK . store (true , Ordering :: SeqCst) ; Ok (()) }) ; let server = builder . build () ; let mut connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_verify (SslVerifyMode :: NONE) ; let s = server . connect_tcp () ; let mut s = connector . build () . configure () . unwrap () . connect ("127.0.0.1" , s) . unwrap () ; s . read_exact (& mut [0]) . unwrap () ; assert ! (CALLED_BACK . load (Ordering :: SeqCst)) ; }
};
}
