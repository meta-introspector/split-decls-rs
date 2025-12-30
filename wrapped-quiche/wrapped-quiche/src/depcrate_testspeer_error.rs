// Generated macro for peer_error (function)
macro_rules! Depcrate_testspeer_error {
() => {
// Module: crate::tests
// Provides: {"peer_error"}
// Dependencies: {}
# [rstest] fn peer_error (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . server . close (false , 0x1234 , b"hello?") , Ok (())) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . peer_error () , Some (& ConnectionError { is_app : false , error_code : 0x1234u64 , reason : b"hello?" . to_vec () })) ; }
};
}
