// Generated macro for coalesce_padding_short (function)
macro_rules! Depcrate_testscoalesce_padding_short {
() => {
// Module: crate::tests
// Provides: {"coalesce_padding_short"}
// Dependencies: {}
# [rstest] fn coalesce_padding_short (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; assert_eq ! (len , MIN_CLIENT_INITIAL_LEN) ; assert_eq ! (pipe . server_recv (& mut buf [.. len]) , Ok (len)) ; let (len , _) = pipe . server . send (& mut buf) . unwrap () ; assert_eq ! (len , MIN_CLIENT_INITIAL_LEN) ; assert_eq ! (pipe . client_recv (& mut buf [.. len]) , Ok (len)) ; let (len , _) = pipe . server . send (& mut buf) . unwrap () ; assert_eq ! (pipe . client_recv (& mut buf [.. len]) , Ok (len)) ; assert ! (pipe . client . is_established ()) ; assert_eq ! (pipe . client . stream_send (4 , b"hello" , true) , Ok (5)) ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; assert_eq ! (len , MIN_CLIENT_INITIAL_LEN) ; assert_eq ! (pipe . server_recv (& mut buf [.. len]) , Ok (len)) ; assert_eq ! (pipe . client . sent_count , pipe . server . recv_count) ; assert_eq ! (pipe . server . sent_count , pipe . client . recv_count) ; }
};
}
