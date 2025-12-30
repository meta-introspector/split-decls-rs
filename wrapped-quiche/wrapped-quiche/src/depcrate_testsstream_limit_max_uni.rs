// Generated macro for stream_limit_max_uni (function)
macro_rules! Depcrate_testsstream_limit_max_uni {
() => {
// Module: crate::tests
// Provides: {"stream_limit_max_uni"}
// Dependencies: {}
# [rstest] fn stream_limit_max_uni (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: MaxStreamsUni { max : MAX_STREAM_ID }] ; let pkt_type = Type :: Short ; assert ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . is_ok ()) ; let frames = [frame :: Frame :: MaxStreamsUni { max : MAX_STREAM_ID + 1 , }] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Err (Error :: InvalidFrame) ,) ; }
};
}
