// Generated macro for flow_control_limit (function)
macro_rules! Depcrate_testsflow_control_limit {
() => {
// Module: crate::tests
// Provides: {"flow_control_limit"}
// Dependencies: {}
# [rstest] fn flow_control_limit (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (b"aaaaaaaaaaaaaaa" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"aaaaaaaaaaaaaaa" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 8 , data : < RangeBuf > :: from (b"a" , 0 , false) , } ,] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Err (Error :: FlowControl) ,) ; }
};
}
