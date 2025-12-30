// Generated macro for peer_cert_chain (function)
macro_rules! Depcrate_testspeer_cert_chain {
() => {
// Module: crate::tests
// Provides: {"peer_cert_chain"}
// Dependencies: {}
# [rstest] fn peer_cert_chain (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . load_cert_chain_from_pem_file ("examples/cert-big.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; let mut pipe = test_utils :: Pipe :: with_server_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; match pipe . client . peer_cert_chain () { Some (c) => assert_eq ! (c . len () , 5) , None => panic ! ("missing server certificate chain") , } }
};
}
