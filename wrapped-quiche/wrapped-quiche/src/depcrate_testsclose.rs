// Generated macro for close (function)
macro_rules! Depcrate_testsclose {
() => {
// Module: crate::tests
// Provides: {"close"}
// Dependencies: {}
# [rstest] fn close (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . close (false , 0x1234 , b"hello?") , Ok (())) ; assert_eq ! (pipe . client . close (false , 0x4321 , b"hello?") , Err (Error :: Done)) ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; let frames = test_utils :: decode_pkt (& mut pipe . server , & mut buf [.. len]) . unwrap () ; assert_eq ! (frames . first () , Some (& frame :: Frame :: ConnectionClose { error_code : 0x1234 , frame_type : 0 , reason : b"hello?" . to_vec () , })) ; }
};
}
