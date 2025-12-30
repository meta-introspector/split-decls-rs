// Generated macro for invalid_initial_server (function)
macro_rules! Depcrate_testsinvalid_initial_server {
() => {
// Module: crate::tests
// Provides: {"invalid_initial_server"}
// Dependencies: {}
# [rstest] # [doc = " Tests that invalid packets received before any other valid ones cause"] # [doc = " the server to close the connection immediately."] fn invalid_initial_server (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; let frames = [frame :: Frame :: Padding { len : 10 }] ; let written = test_utils :: encode_pkt (& mut pipe . client , Type :: Initial , & frames , & mut buf ,) . unwrap () ; buf [written - 1] = ! buf [written - 1] ; assert_eq ! (pipe . server . timeout () , None) ; assert_eq ! (pipe . server_recv (& mut buf [.. written]) , Err (Error :: CryptoFail)) ; assert ! (pipe . server . is_closed ()) ; }
};
}
