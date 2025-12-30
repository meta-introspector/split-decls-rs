// Generated macro for send_ack_eliciting_causes_ping (function)
macro_rules! Depcrate_testssend_ack_eliciting_causes_ping {
() => {
// Module: crate::tests
// Provides: {"send_ack_eliciting_causes_ping"}
// Dependencies: {}
# [rstest] fn send_ack_eliciting_causes_ping (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; pipe . server . send_ack_eliciting () . unwrap () ; let mut buf = [0 ; 1500] ; let (len , _) = pipe . server . send (& mut buf) . unwrap () ; let frames = test_utils :: decode_pkt (& mut pipe . client , & mut buf [.. len]) . unwrap () ; let mut iter = frames . iter () ; assert_eq ! (iter . next () , Some (& frame :: Frame :: Ping { mtu_probe : None })) ; }
};
}
