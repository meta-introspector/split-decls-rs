// Generated macro for app_close_by_server_during_handshake_not_established (function)
macro_rules! Depcrate_testsapp_close_by_server_during_handshake_not_established {
() => {
// Module: crate::tests
// Provides: {"app_close_by_server_during_handshake_not_established"}
// Dependencies: {}
# [rstest] fn app_close_by_server_during_handshake_not_established (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . client) . unwrap () ; test_utils :: process_flight (& mut pipe . server , flight) . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . server) . unwrap () ; assert ! (! pipe . client . is_established () && ! pipe . server . is_established ()) ; pipe . server . close (true , 123 , b"fail whale") . unwrap () ; test_utils :: process_flight (& mut pipe . client , flight) . unwrap () ; assert ! (pipe . client . is_established ()) ; pipe . client . stream_send (0 , b"badauthtoken" , true) . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . client) . unwrap () ; test_utils :: process_flight (& mut pipe . server , flight) . unwrap () ; assert ! (! pipe . server . is_established ()) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . server . local_error () , Some (& ConnectionError { is_app : false , error_code : 0x0c , reason : vec ! [] , })) ; assert_eq ! (pipe . client . peer_error () , Some (& ConnectionError { is_app : false , error_code : 0x0c , reason : vec ! [] , })) ; }
};
}
