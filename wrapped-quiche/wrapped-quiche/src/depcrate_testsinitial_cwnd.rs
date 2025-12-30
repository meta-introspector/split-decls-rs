// Generated macro for initial_cwnd (function)
macro_rules! Depcrate_testsinitial_cwnd {
() => {
// Module: crate::tests
// Provides: {"initial_cwnd"}
// Dependencies: {}
# [rstest] fn initial_cwnd (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) -> Result < () > { const CUSTOM_INITIAL_CONGESTION_WINDOW_PACKETS : usize = 30 ; let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . set_initial_congestion_window_packets (CUSTOM_INITIAL_CONGESTION_WINDOW_PACKETS ,) ; config . load_cert_chain_from_pem_file ("examples/cert.crt") ? ; config . load_priv_key_from_pem_file ("examples/cert.key") ? ; config . set_application_protos (& [b"proto1" , b"proto2"]) ? ; config . set_initial_max_data (1000000) ; config . set_initial_max_stream_data_bidi_local (15) ; config . set_initial_max_stream_data_bidi_remote (15) ; config . set_initial_max_stream_data_uni (10) ; config . set_initial_max_streams_bidi (3) ; config . set_initial_max_streams_uni (3) ; config . set_max_idle_timeout (180_000) ; config . verify_peer (false) ; config . set_ack_delay_exponent (8) ; let mut pipe = test_utils :: Pipe :: with_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; if cc_algorithm_name == "cubic" { assert_eq ! (pipe . server . tx_cap , CUSTOM_INITIAL_CONGESTION_WINDOW_PACKETS * 1200) ; } else { let expected = CUSTOM_INITIAL_CONGESTION_WINDOW_PACKETS * 1200 + if cfg ! (feature = "openssl") { 1463 } else { 1447 } ; assert ! (pipe . server . tx_cap >= expected , "{} vs {}" , pipe . server . tx_cap , expected) ; assert ! (pipe . server . tx_cap <= expected + 1 , "{} vs {}" , pipe . server . tx_cap , expected + 1) ; } Ok (()) }
};
}
