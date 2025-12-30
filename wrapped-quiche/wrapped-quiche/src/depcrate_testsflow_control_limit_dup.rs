// Generated macro for flow_control_limit_dup (function)
macro_rules! Depcrate_testsflow_control_limit_dup {
() => {
// Module: crate::tests
// Provides: {"flow_control_limit_dup"}
// Dependencies: {}
# [rstest] fn flow_control_limit_dup (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (b"aaaaaaaaaaaaaa" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (b"aaaaaaaaaaaaaaa" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 8 , data : < RangeBuf > :: from (b"aaaaaaaaaaaaaaa" , 0 , false) , } ,] ; let pkt_type = Type :: Short ; assert ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . is_ok ()) ; }
};
}
