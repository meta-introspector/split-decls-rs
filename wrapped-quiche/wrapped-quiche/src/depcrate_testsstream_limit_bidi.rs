// Generated macro for stream_limit_bidi (function)
macro_rules! Depcrate_testsstream_limit_bidi {
() => {
// Module: crate::tests
// Provides: {"stream_limit_bidi"}
// Dependencies: {}
# [rstest] fn stream_limit_bidi (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 8 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 12 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 16 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 20 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 24 , data : < RangeBuf > :: from (b"a" , 0 , false) , } , frame :: Frame :: Stream { stream_id : 28 , data : < RangeBuf > :: from (b"a" , 0 , false) , } ,] ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) , Err (Error :: StreamLimit) ,) ; }
};
}
