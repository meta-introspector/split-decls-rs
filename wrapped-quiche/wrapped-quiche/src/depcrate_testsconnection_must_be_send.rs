// Generated macro for connection_must_be_send (function)
macro_rules! Depcrate_testsconnection_must_be_send {
() => {
// Module: crate::tests
// Provides: {"connection_must_be_send"}
// Dependencies: {}
# [rstest] fn connection_must_be_send (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; check_send (& mut pipe . client) ; }
};
}
