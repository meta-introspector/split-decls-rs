// Generated macro for stream_zero_length_non_fin (function)
macro_rules! Depcrate_testsstream_zero_length_non_fin {
() => {
// Module: crate::tests
// Provides: {"stream_zero_length_non_fin"}
// Dependencies: {}
# [rstest] # [doc = " Tests that the stream gets created with stream_send() even if there's"] # [doc = " no data in the buffer and the fin flag is not set."] fn stream_zero_length_non_fin (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"" , false) , Ok (0)) ; assert_eq ! (pipe . client . streams . len () , 1) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . server . readable () ; assert ! (r . next () . is_none ()) ; }
};
}
