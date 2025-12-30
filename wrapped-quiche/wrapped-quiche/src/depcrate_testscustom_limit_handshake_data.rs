// Generated macro for custom_limit_handshake_data (function)
macro_rules! Depcrate_testscustom_limit_handshake_data {
() => {
// Module: crate::tests
// Provides: {"custom_limit_handshake_data"}
// Dependencies: {}
# [rstest] fn custom_limit_handshake_data (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { const CUSTOM_AMPLIFICATION_FACTOR : usize = 2 ; let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . load_cert_chain_from_pem_file ("examples/cert-big.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . set_max_amplification_factor (CUSTOM_AMPLIFICATION_FACTOR) ; let mut pipe = test_utils :: Pipe :: with_server_config (& mut config) . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . client) . unwrap () ; let client_sent = flight . iter () . fold (0 , | out , p | out + p . 0 . len ()) ; test_utils :: process_flight (& mut pipe . server , flight) . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . server) . unwrap () ; let server_sent = flight . iter () . fold (0 , | out , p | out + p . 0 . len ()) ; assert_eq ! (server_sent , client_sent * CUSTOM_AMPLIFICATION_FACTOR) ; }
};
}
