// Generated macro for peer_cert (function)
macro_rules! Depcrate_testspeer_cert {
() => {
// Module: crate::tests
// Provides: {"peer_cert"}
// Dependencies: {}
# [rstest] fn peer_cert (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; match pipe . client . peer_cert () { Some (c) => assert_eq ! (c . len () , 753) , None => panic ! ("missing server certificate") , } }
};
}
