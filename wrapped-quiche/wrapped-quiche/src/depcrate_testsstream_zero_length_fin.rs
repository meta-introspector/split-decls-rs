// Generated macro for stream_zero_length_fin (function)
macro_rules! Depcrate_testsstream_zero_length_fin {
() => {
// Module: crate::tests
// Provides: {"stream_zero_length_fin"}
// Dependencies: {}
# [rstest] # [doc = " Tests that the stream's fin flag is properly flushed even if there's no"] # [doc = " data in the buffer, and that the buffer becomes readable on the other"] # [doc = " side."] fn stream_zero_length_fin (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"aaaaaaaaaaaaaaa" , false) , Ok (15)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . server . readable () ; assert_eq ! (r . next () , Some (0)) ; assert ! (r . next () . is_none ()) ; let mut b = [0 ; 15] ; pipe . server . stream_recv (0 , & mut b) . unwrap () ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"" , true) , Ok (0)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . server . readable () ; assert_eq ! (r . next () , Some (0)) ; assert ! (r . next () . is_none ()) ; let mut b = [0 ; 15] ; pipe . server . stream_recv (0 , & mut b) . unwrap () ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"" , true) , Ok (0)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . server . readable () ; assert_eq ! (r . next () , None) ; }
};
}
