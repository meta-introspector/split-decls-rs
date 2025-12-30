// Generated macro for empty_payload (function)
macro_rules! Depcrate_testsempty_payload {
() => {
// Module: crate::tests
// Provides: {"empty_payload"}
// Dependencies: {}
# [rstest] fn empty_payload (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let pkt_type = Type :: Short ; assert_eq ! (pipe . send_pkt_to_server (pkt_type , & [] , & mut buf) , Err (Error :: InvalidPacket)) ; }
};
}
