// Generated macro for stop_sending_fin (function)
macro_rules! Depcrate_testsstop_sending_fin {
() => {
// Module: crate::tests
// Provides: {"stop_sending_fin"}
// Dependencies: {}
# [rstest] fn stop_sending_fin (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut b = [0 ; 15] ; let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (4 , b"hello" , true) , Ok (5)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . server . readable () ; assert_eq ! (r . next () , Some (4)) ; assert_eq ! (r . next () , None) ; assert_eq ! (pipe . server . stream_recv (4 , & mut b) , Ok ((5 , true))) ; assert ! (pipe . server . stream_finished (4)) ; let mut r = pipe . server . readable () ; assert_eq ! (r . next () , None) ; let mut r = pipe . server . writable () ; assert_eq ! (r . next () , Some (4)) ; assert_eq ! (r . next () , None) ; assert_eq ! (pipe . server . stream_send (4 , b"world" , false) , Ok (5)) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . server . stream_send (4 , b"world" , true) , Ok (5)) ; let frames = [frame :: Frame :: StopSending { stream_id : 4 , error_code : 42 , }] ; let pkt_type = Type :: Short ; let len = pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . unwrap () ; let frames = test_utils :: decode_pkt (& mut pipe . client , & mut buf [.. len]) . unwrap () ; let mut iter = frames . iter () ; iter . next () ; assert_eq ! (iter . next () , Some (& frame :: Frame :: ResetStream { stream_id : 4 , error_code : 42 , final_size : 5 , })) ; assert_eq ! (iter . next () , None) ; }
};
}
