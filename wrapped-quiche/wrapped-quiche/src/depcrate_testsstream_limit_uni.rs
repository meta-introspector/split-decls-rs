// Generated macro for stream_limit_uni (function)
macro_rules! Depcrate_testsstream_limit_uni {
() => {
// Module: crate::tests
// Provides: {"stream_limit_uni"}
// Dependencies: {}
# [rstest] fn stream_limit_uni (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 2 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 6 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 10 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 14 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 18 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 22 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 26 , data : < RangeBuf > :: from (b"a" , 0 , false) , } ,] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Err (Error :: StreamLimit) ,) ; }
};
}
