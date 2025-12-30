// Generated macro for update_key_request (function)
macro_rules! Depcrate_testsupdate_key_request {
() => {
// Module: crate::tests
// Provides: {"update_key_request"}
// Dependencies: {}
# [rstest] fn update_key_request (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut b = [0 ; 15] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client_update_key () , Ok (())) ; assert_eq ! (pipe . client . stream_send (4 , b"hello" , false) , Ok (5)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . server . readable () ; assert_eq ! (r . next () , Some (4)) ; assert_eq ! (r . next () , None) ; assert_eq ! (pipe . server . stream_recv (4 , & mut b) , Ok ((5 , false))) ; assert_eq ! (& b [.. 5] , b"hello") ; assert ! (pipe . server . crypto_ctx [packet :: Epoch :: Application] . key_update . as_ref () . unwrap () . update_acked) ; assert_eq ! (pipe . server . stream_send (4 , b"world" , false) , Ok (5)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . client . readable () ; assert_eq ! (r . next () , Some (4)) ; assert_eq ! (r . next () , None) ; assert_eq ! (pipe . client . stream_recv (4 , & mut b) , Ok ((5 , false))) ; assert_eq ! (& b [.. 5] , b"world") ; for _ in 0 .. 10 { assert_eq ! (pipe . server . stream_send (4 , b"world" , false) , Ok (5)) ; assert_eq ! (pipe . advance () , Ok (())) ; let mut r = pipe . client . readable () ; assert_eq ! (r . next () , Some (4)) ; assert_eq ! (r . next () , None) ; assert_eq ! (pipe . client . stream_recv (4 , & mut b) , Ok ((5 , false))) ; assert_eq ! (& b [.. 5] , b"world") ; } }
};
}
