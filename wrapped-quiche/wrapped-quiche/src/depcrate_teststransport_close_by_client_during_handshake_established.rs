// Generated macro for transport_close_by_client_during_handshake_established (function)
macro_rules! Depcrate_teststransport_close_by_client_during_handshake_established {
() => {
// Module: crate::tests
// Provides: {"transport_close_by_client_during_handshake_established"}
// Dependencies: {}
# [rstest] fn transport_close_by_client_during_handshake_established (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . client) . unwrap () ; test_utils :: process_flight (& mut pipe . server , flight) . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . server) . unwrap () ; assert ! (! pipe . client . is_established () && ! pipe . server . is_established ()) ; test_utils :: process_flight (& mut pipe . client , flight) . unwrap () ; assert ! (pipe . client . is_established ()) ; pipe . client . close (false , 123 , b"connection close") . unwrap () ; let flight = test_utils :: emit_flight (& mut pipe . client) . unwrap () ; test_utils :: process_flight (& mut pipe . server , flight) . unwrap () ; assert_eq ! (pipe . server . peer_error () , Some (& ConnectionError { is_app : false , error_code : 123 , reason : b"connection close" . to_vec () })) ; assert_eq ! (pipe . client . local_error () , Some (& ConnectionError { is_app : false , error_code : 123 , reason : b"connection close" . to_vec () })) ; }
};
}
