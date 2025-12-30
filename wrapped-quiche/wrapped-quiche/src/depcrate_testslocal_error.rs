// Generated macro for local_error (function)
macro_rules! Depcrate_testslocal_error {
() => {
// Module: crate::tests
// Provides: {"local_error"}
// Dependencies: {}
# [rstest] fn local_error (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . server . local_error () , None) ; assert_eq ! (pipe . server . close (true , 0x1234 , b"hello!") , Ok (())) ; assert_eq ! (pipe . server . local_error () , Some (& ConnectionError { is_app : true , error_code : 0x1234u64 , reason : b"hello!" . to_vec () })) ; }
};
}
