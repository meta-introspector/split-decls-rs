// Generated macro for client_sent_new_token (function)
macro_rules! Depcrate_testsclient_sent_new_token {
() => {
// Module: crate::tests
// Provides: {"client_sent_new_token"}
// Dependencies: {}
# [rstest] # [doc = " Tests that a NEW_TOKEN frame sent by client is detected as an error."] fn client_sent_new_token (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = vec ! [frame :: Frame :: NewToken { token : vec ! [1 , 2 , 3] , }] ; let pkt_type = Type :: Short ; let written = test_utils :: encode_pkt (& mut pipe . client , pkt_type , & frames , & mut buf) . unwrap () ; assert_eq ! (pipe . server_recv (& mut buf [.. written]) , Err (Error :: InvalidPacket)) ; }
};
}
