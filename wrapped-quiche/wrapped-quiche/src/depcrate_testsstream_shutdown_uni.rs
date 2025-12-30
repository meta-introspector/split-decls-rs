// Generated macro for stream_shutdown_uni (function)
macro_rules! Depcrate_testsstream_shutdown_uni {
() => {
// Module: crate::tests
// Provides: {"stream_shutdown_uni"}
// Dependencies: {}
# [rstest] fn stream_shutdown_uni (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (2 , b"hello, world" , false) , Ok (10)) ; assert_eq ! (pipe . server . stream_send (3 , b"hello, world" , false) , Ok (10)) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_shutdown (2 , Shutdown :: Write , 42) , Ok (())) ; assert_eq ! (pipe . client . stream_shutdown (2 , Shutdown :: Read , 42) , Err (Error :: InvalidStreamState (2))) ; assert_eq ! (pipe . client . stream_shutdown (3 , Shutdown :: Write , 42) , Err (Error :: InvalidStreamState (3))) ; assert_eq ! (pipe . client . stream_shutdown (3 , Shutdown :: Read , 42) , Ok (())) ; }
};
}
