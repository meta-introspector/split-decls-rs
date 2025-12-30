// Generated macro for handshake (function)
macro_rules! Depcrate_testshandshake {
() => {
// Module: crate::tests
// Provides: {"handshake"}
// Dependencies: {}
# [rstest] fn handshake (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . application_proto () , pipe . server . application_proto ()) ; assert_eq ! (pipe . server . server_name () , Some ("quic.tech")) ; }
};
}
