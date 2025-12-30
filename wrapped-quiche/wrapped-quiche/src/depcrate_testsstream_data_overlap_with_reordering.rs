// Generated macro for stream_data_overlap_with_reordering (function)
macro_rules! Depcrate_testsstream_data_overlap_with_reordering {
() => {
// Module: crate::tests
// Provides: {"stream_data_overlap_with_reordering"}
// Dependencies: {}
# [rstest] fn stream_data_overlap_with_reordering (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (b"aaaaa" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (b"ccccc" , 6 , false) , } , frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (b"bbbbb" , 3 , false) , } ,] ; let pkt_type = Type :: Short ; assert ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . is_ok ()) ; let mut b = [0 ; 15] ; assert_eq ! (pipe . server . stream_recv (0 , & mut b) , Ok ((11 , false))) ; assert_eq ! (& b [.. 11] , b"aaaaabccccc") ; assert_eq ! (pipe . server . flow_control . consumed () , pipe . server . rx_data) ; }
};
}
