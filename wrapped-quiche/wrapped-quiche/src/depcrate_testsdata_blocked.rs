// Generated macro for data_blocked (function)
macro_rules! Depcrate_testsdata_blocked {
() => {
// Module: crate::tests
// Provides: {"data_blocked"}
// Dependencies: {}
# [rstest] fn data_blocked (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"aaaaaaaaaa" , false) , Ok (10)) ; assert_eq ! (pipe . client . blocked_limit , None) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_send (4 , b"aaaaaaaaaa" , false) , Ok (10)) ; assert_eq ! (pipe . client . blocked_limit , None) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_send (8 , b"aaaaaaaaaaa" , false) , Ok (10)) ; assert_eq ! (pipe . client . blocked_limit , Some (30)) ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; assert_eq ! (pipe . client . blocked_limit , None) ; let frames = test_utils :: decode_pkt (& mut pipe . server , & mut buf [.. len]) . unwrap () ; let mut iter = frames . iter () ; assert_eq ! (iter . next () , Some (& frame :: Frame :: DataBlocked { limit : 30 })) ; assert_eq ! (iter . next () , Some (& frame :: Frame :: Stream { stream_id : 8 , data : < RangeBuf >:: from (b"aaaaaaaaaa" , 0 , false) , })) ; assert_eq ! (iter . next () , None) ; }
};
}
