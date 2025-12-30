// Generated macro for app_close_by_client (function)
macro_rules! Depcrate_testsapp_close_by_client {
() => {
// Module: crate::tests
// Provides: {"app_close_by_client"}
// Dependencies: {}
# [rstest] fn app_close_by_client (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . close (true , 0x1234 , b"hello!") , Ok (())) ; assert_eq ! (pipe . client . close (true , 0x4321 , b"hello!") , Err (Error :: Done)) ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; let frames = test_utils :: decode_pkt (& mut pipe . server , & mut buf [.. len]) . unwrap () ; assert_eq ! (frames . first () , Some (& frame :: Frame :: ApplicationClose { error_code : 0x1234 , reason : b"hello!" . to_vec () , })) ; }
};
}
