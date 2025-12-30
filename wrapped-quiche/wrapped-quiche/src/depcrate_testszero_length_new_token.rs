// Generated macro for zero_length_new_token (function)
macro_rules! Depcrate_testszero_length_new_token {
() => {
// Module: crate::tests
// Provides: {"zero_length_new_token"}
// Dependencies: {}
# [rstest] # [doc = " Tests that a zero-length NEW_TOKEN frame is detected as an error."] fn zero_length_new_token (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = vec ! [frame :: Frame :: NewToken { token : vec ! [] }] ; let pkt_type = Type :: Short ; let written = test_utils :: encode_pkt (& mut pipe . server , pkt_type , & frames , & mut buf) . unwrap () ; assert_eq ! (pipe . client_recv (& mut buf [.. written]) , Err (Error :: InvalidFrame)) ; }
};
}
