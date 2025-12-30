// Generated macro for stream_left_bidi (function)
macro_rules! Depcrate_testsstream_left_bidi {
() => {
// Module: crate::tests
// Provides: {"stream_left_bidi"}
// Dependencies: {}
# [rstest] fn stream_left_bidi (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (3 , pipe . client . peer_streams_left_bidi ()) ; assert_eq ! (3 , pipe . server . peer_streams_left_bidi ()) ; pipe . server . stream_send (1 , b"a" , false) . ok () ; assert_eq ! (2 , pipe . server . peer_streams_left_bidi ()) ; pipe . server . stream_send (5 , b"a" , false) . ok () ; assert_eq ! (1 , pipe . server . peer_streams_left_bidi ()) ; pipe . server . stream_send (9 , b"a" , false) . ok () ; assert_eq ! (0 , pipe . server . peer_streams_left_bidi ()) ; let frames = [frame :: Frame :: MaxStreamsBidi { max : MAX_STREAM_ID }] ; let pkt_type = Type :: Short ; assert ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . is_ok ()) ; assert_eq ! (MAX_STREAM_ID - 3 , pipe . server . peer_streams_left_bidi ()) ; }
};
}
