// Generated macro for app_close_by_server_during_handshake_private_key_failure (function)
macro_rules! Depcrate_testsapp_close_by_server_during_handshake_private_key_failure {
() => {
// Module: crate::tests
// Provides: {"app_close_by_server_during_handshake_private_key_failure"}
// Dependencies: {}
# [cfg (not (feature = "openssl"))] # [rstest] fn app_close_by_server_during_handshake_private_key_failure (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; pipe . server . handshake . set_failing_private_key_method () ; let flight = test_utils :: emit_flight (& mut pipe . client) . unwrap () ; assert_eq ! (test_utils :: process_flight (& mut pipe . server , flight) , Err (Error :: TlsFail)) ; let flight = test_utils :: emit_flight (& mut pipe . server) . unwrap () ; assert ! (! pipe . server . is_established ()) ; assert ! (! pipe . client . is_established ()) ; assert_eq ! (pipe . server . close (true , 123 , b"fail whale") , Err (Error :: Done)) ; test_utils :: process_flight (& mut pipe . client , flight) . unwrap () ; assert_eq ! (pipe . client . close (true , 123 , b"fail whale") , Err (Error :: Done)) ; assert ! (! pipe . server . is_established ()) ; assert ! (! pipe . client . is_established ()) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . server . local_error () , Some (& ConnectionError { is_app : false , error_code : 0x01 , reason : vec ! [] , })) ; assert_eq ! (pipe . client . peer_error () , Some (& ConnectionError { is_app : false , error_code : 0x01 , reason : vec ! [] , })) ; }
};
}
