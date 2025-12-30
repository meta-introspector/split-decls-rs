// Generated macro for flow_control_limit_send (function)
macro_rules! Depcrate_testsflow_control_limit_send {
() => {
// Module: crate::tests
// Provides: {"flow_control_limit_send"}
// Dependencies: {}
# [rstest] # [doc = " Tests that we don't exceed the per-connection flow control limit set by"] # [doc = " the peer."] fn flow_control_limit_send (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"aaaaaaaaaaaaaaa" , false) , Ok (15)) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_send (4 , b"aaaaaaaaaaaaaaa" , false) , Ok (15)) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_send (8 , b"a" , false) , Err (Error :: Done)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . server . readable () ; assert ! (r . next () . is_some ()) ; assert ! (r . next () . is_some ()) ; assert ! (r . next () . is_none ()) ; }
};
}
