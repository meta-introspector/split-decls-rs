// Generated macro for empty_stream_frame (function)
macro_rules! Depcrate_testsempty_stream_frame {
() => {
// Module: crate::tests
// Provides: {"empty_stream_frame"}
// Dependencies: {}
# [rstest] fn empty_stream_frame (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"aaaaa" , 0 , false) , }] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Ok (39)) ; let mut readable = pipe . server . readable () ; assert_eq ! (readable . next () , Some (4)) ; assert_eq ! (pipe . server . stream_recv (4 , & mut buf) , Ok ((5 , false))) ; let frames = [frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"" , 5 , true) , }] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Ok (39)) ; let mut readable = pipe . server . readable () ; assert_eq ! (readable . next () , Some (4)) ; assert_eq ! (pipe . server . stream_recv (4 , & mut buf) , Ok ((0 , true))) ; let frames = [frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"" , 15 , true) , }] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Err (Error :: FinalSize)) ; }
};
}
