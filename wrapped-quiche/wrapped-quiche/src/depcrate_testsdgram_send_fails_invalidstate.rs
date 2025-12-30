// Generated macro for dgram_send_fails_invalidstate (function)
macro_rules! Depcrate_testsdgram_send_fails_invalidstate {
() => {
// Module: crate::tests
// Provides: {"dgram_send_fails_invalidstate"}
// Dependencies: {}
# [rstest] fn dgram_send_fails_invalidstate (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . dgram_send (b"hello, world") , Err (Error :: InvalidState)) ; }
};
}
