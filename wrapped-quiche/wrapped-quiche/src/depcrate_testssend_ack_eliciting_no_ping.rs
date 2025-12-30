// Generated macro for send_ack_eliciting_no_ping (function)
macro_rules! Depcrate_testssend_ack_eliciting_no_ping {
() => {
// Module: crate::tests
// Provides: {"send_ack_eliciting_no_ping"}
// Dependencies: {}
# [rstest] fn send_ack_eliciting_no_ping (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; pipe . server . send_ack_eliciting () . unwrap () ; assert_eq ! (pipe . server . stream_send (1 , b"a" , false) , Ok (1)) ; let mut buf = [0 ; 1500] ; let (len , _) = pipe . server . send (& mut buf) . unwrap () ; let frames = test_utils :: decode_pkt (& mut pipe . client , & mut buf [.. len]) . unwrap () ; let mut iter = frames . iter () ; assert ! (matches ! (iter . next () , Some (& frame :: Frame :: Stream { stream_id : 1 , data : _ }))) ; assert ! (iter . next () . is_none ()) ; }
};
}
