// Generated macro for client_discard_unknown_address (function)
macro_rules! Depcrate_testsclient_discard_unknown_address {
() => {
// Module: crate::tests
// Provides: {"client_discard_unknown_address"}
// Dependencies: {}
# [rstest] fn client_discard_unknown_address (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . verify_peer (false) ; config . set_initial_max_data (30) ; config . set_initial_max_stream_data_uni (10) ; config . set_initial_max_streams_uni (3) ; let mut pipe = test_utils :: Pipe :: with_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . server . stream_send (3 , b"a" , true) , Ok (1)) ; let mut flight = test_utils :: emit_flight (& mut pipe . server) . expect ("no packet") ; flight . iter_mut () . for_each (| (_ , si) | si . from = "127.0.0.1:9292" . parse () . unwrap ()) ; assert_eq ! (test_utils :: process_flight (& mut pipe . client , flight) , Ok (())) ; assert_eq ! (pipe . client . paths . len () , 1) ; }
};
}
