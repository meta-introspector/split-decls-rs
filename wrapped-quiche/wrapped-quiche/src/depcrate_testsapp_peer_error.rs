// Generated macro for app_peer_error (function)
macro_rules! Depcrate_testsapp_peer_error {
() => {
// Module: crate::tests
// Provides: {"app_peer_error"}
// Dependencies: {}
# [rstest] fn app_peer_error (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . server . close (true , 0x1234 , b"hello!") , Ok (())) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . peer_error () , Some (& ConnectionError { is_app : true , error_code : 0x1234u64 , reason : b"hello!" . to_vec () })) ; }
};
}
