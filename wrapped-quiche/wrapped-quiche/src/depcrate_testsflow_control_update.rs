// Generated macro for flow_control_update (function)
macro_rules! Depcrate_testsflow_control_update {
() => {
// Module: crate::tests
// Provides: {"flow_control_update"}
// Dependencies: {}
# [rstest] fn flow_control_update (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (b"aaaaaaaaaaaaaaa" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"a" , 0 , false) , } ,] ; let pkt_type = Type :: Short ; assert ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . is_ok ()) ; pipe . server . stream_recv (0 , & mut buf) . unwrap () ; pipe . server . stream_recv (4 , & mut buf) . unwrap () ; let frames = [frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"a" , 1 , false) , }] ; let len = pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . unwrap () ; assert ! (len > 0) ; let frames = test_utils :: decode_pkt (& mut pipe . client , & mut buf [.. len]) . unwrap () ; let mut iter = frames . iter () ; iter . next () . unwrap () ; assert_eq ! (iter . next () , Some (& frame :: Frame :: MaxStreamData { stream_id : 0 , max : 30 })) ; assert_eq ! (iter . next () , Some (& frame :: Frame :: MaxData { max : 61 })) ; }
};
}
