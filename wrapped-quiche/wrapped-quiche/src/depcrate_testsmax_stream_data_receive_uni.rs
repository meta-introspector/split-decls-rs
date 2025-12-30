// Generated macro for max_stream_data_receive_uni (function)
macro_rules! Depcrate_testsmax_stream_data_receive_uni {
() => {
// Module: crate::tests
// Provides: {"max_stream_data_receive_uni"}
// Dependencies: {}
# [rstest] # [doc = " Tests that receiving a MAX_STREAM_DATA frame for a receive-only"] # [doc = " unidirectional stream is forbidden."] fn max_stream_data_receive_uni (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (2 , b"hello" , false) , Ok (5)) ; assert_eq ! (pipe . advance () , Ok (())) ; let frames = [frame :: Frame :: MaxStreamData { stream_id : 2 , max : 1024 , }] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Err (Error :: InvalidStreamState (2)) ,) ; }
};
}
