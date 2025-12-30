// Generated macro for stream_send_on_32bit_arch (function)
macro_rules! Depcrate_testsstream_send_on_32bit_arch {
() => {
// Module: crate::tests
// Provides: {"stream_send_on_32bit_arch"}
// Dependencies: {}
# [rstest] fn stream_send_on_32bit_arch (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . set_initial_max_data (2_u64 . pow (32) + 5) ; config . set_initial_max_stream_data_bidi_local (15) ; config . set_initial_max_stream_data_bidi_remote (15) ; config . set_initial_max_stream_data_uni (10) ; config . set_initial_max_streams_bidi (3) ; config . set_initial_max_streams_uni (0) ; config . verify_peer (false) ; let mut pipe = test_utils :: Pipe :: with_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (4 , b"hello, world" , true) , Ok (12)) ; assert_eq ! (pipe . advance () , Ok (())) ; assert ! (! pipe . server . stream_finished (4)) ; }
};
}
