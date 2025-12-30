// Generated macro for consecutive_non_ack_eliciting (function)
macro_rules! Depcrate_testsconsecutive_non_ack_eliciting {
() => {
// Module: crate::tests
// Provides: {"consecutive_non_ack_eliciting"}
// Dependencies: {}
# [rstest] fn consecutive_non_ack_eliciting (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Ping { mtu_probe : None }] ; let pkt_type = Type :: Short ; for _ in 0 .. 24 { let len = pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . unwrap () ; assert ! (len > 0) ; let frames = test_utils :: decode_pkt (& mut pipe . client , & mut buf [.. len]) . unwrap () ; assert ! (frames . iter () . all (| frame | matches ! (frame , frame :: Frame :: ACK { .. })) , "ACK only") ; } let len = pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . unwrap () ; assert ! (len > 0) ; let frames = test_utils :: decode_pkt (& mut pipe . client , & mut buf [.. len]) . unwrap () ; assert ! (frames . iter () . any (| frame | matches ! (frame , frame :: Frame :: Ping { mtu_probe : None })) , "found a PING") ; }
};
}
