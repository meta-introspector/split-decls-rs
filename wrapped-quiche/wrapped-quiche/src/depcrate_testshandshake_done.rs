// Generated macro for handshake_done (function)
macro_rules! Depcrate_testshandshake_done {
() => {
// Module: crate::tests
// Provides: {"handshake_done"}
// Dependencies: {}
# [rstest] fn handshake_done (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; pipe . server . handshake . set_options (0x0000_4000) ; assert_eq ! (pipe . handshake () , Ok (())) ; assert ! (pipe . server . handshake_done_sent) ; }
};
}
