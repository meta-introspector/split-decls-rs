// Generated macro for streamio (function)
macro_rules! Depcrate_testsstreamio {
() => {
// Module: crate::tests
// Provides: {"streamio"}
// Dependencies: {}
# [rstest] fn streamio (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (4 , b"hello, world" , true) , Ok (12)) ; assert_eq ! (pipe . advance () , Ok (())) ; assert ! (! pipe . server . stream_finished (4)) ; let mut r = pipe . server . readable () ; assert_eq ! (r . next () , Some (4)) ; assert_eq ! (r . next () , None) ; let mut b = [0 ; 15] ; assert_eq ! (pipe . server . stream_recv (4 , & mut b) , Ok ((12 , true))) ; assert_eq ! (& b [.. 12] , b"hello, world") ; assert ! (pipe . server . stream_finished (4)) ; }
};
}
