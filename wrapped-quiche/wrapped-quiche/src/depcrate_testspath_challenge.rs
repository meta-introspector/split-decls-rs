// Generated macro for path_challenge (function)
macro_rules! Depcrate_testspath_challenge {
() => {
// Module: crate::tests
// Provides: {"path_challenge"}
// Dependencies: {}
# [rstest] fn path_challenge (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: PathChallenge { data : [0xba ; 8] }] ; let pkt_type = Type :: Short ; let len = pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . unwrap () ; assert ! (len > 0) ; let frames = test_utils :: decode_pkt (& mut pipe . client , & mut buf [.. len]) . unwrap () ; let mut iter = frames . iter () ; iter . next () . unwrap () ; assert_eq ! (iter . next () , Some (& frame :: Frame :: PathResponse { data : [0xba ; 8] })) ; }
};
}
