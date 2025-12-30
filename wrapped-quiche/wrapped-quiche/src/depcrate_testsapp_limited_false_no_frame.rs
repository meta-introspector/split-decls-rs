// Generated macro for app_limited_false_no_frame (function)
macro_rules! Depcrate_testsapp_limited_false_no_frame {
() => {
// Module: crate::tests
// Provides: {"app_limited_false_no_frame"}
// Dependencies: {}
# [rstest] fn app_limited_false_no_frame (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . set_initial_max_data (50000) ; config . set_initial_max_stream_data_bidi_local (50000) ; config . set_initial_max_stream_data_bidi_remote (50000) ; config . set_max_recv_udp_payload_size (1405) ; config . verify_peer (false) ; let mut pipe = test_utils :: Pipe :: with_client_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"a" , true) , Ok (1)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut b = [0 ; 15] ; pipe . server . stream_recv (0 , & mut b) . unwrap () ; assert_eq ! (pipe . advance () , Ok (())) ; let send_buf1 = [0 ; 20000] ; assert_eq ! (pipe . server . stream_send (0 , & send_buf1 , false) , Ok (12000)) ; test_utils :: emit_flight (& mut pipe . server) . ok () ; assert ! (! pipe . server . paths . get_active () . expect ("no active") . recovery . app_limited ()) ; }
};
}
