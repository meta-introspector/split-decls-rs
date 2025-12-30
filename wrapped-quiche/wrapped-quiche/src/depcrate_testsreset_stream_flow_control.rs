// Generated macro for reset_stream_flow_control (function)
macro_rules! Depcrate_testsreset_stream_flow_control {
() => {
// Module: crate::tests
// Provides: {"reset_stream_flow_control"}
// Dependencies: {}
# [rstest] # [doc = " Tests that RESET_STREAM frames exceeding the connection-level flow"] # [doc = " control limit cause an error."] fn reset_stream_flow_control (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 0 , data : < RangeBuf > :: from (& [1 ; 15] , 0 , false) , } , frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: ResetStream { stream_id : 4 , error_code : 0 , final_size : 15 , } , frame :: Frame :: Stream { stream_id : 8 , data : < RangeBuf > :: from (b"a" , 0 , false) , } ,] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Err (Error :: FlowControl) ,) ; }
};
}
