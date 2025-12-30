// Generated macro for recv_empty_buffer (function)
macro_rules! Depcrate_testsrecv_empty_buffer {
() => {
// Module: crate::tests
// Provides: {"recv_empty_buffer"}
// Dependencies: {}
# [rstest] fn recv_empty_buffer (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . server_recv (& mut buf [.. 0]) , Err (Error :: BufferTooShort)) ; }
};
}
