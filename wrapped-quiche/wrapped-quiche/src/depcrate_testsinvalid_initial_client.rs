// Generated macro for invalid_initial_client (function)
macro_rules! Depcrate_testsinvalid_initial_client {
() => {
// Module: crate::tests
// Provides: {"invalid_initial_client"}
// Dependencies: {}
# [rstest] # [doc = " Tests that invalid Initial packets received to cause"] # [doc = " the client to close the connection immediately."] fn invalid_initial_client (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; assert_eq ! (pipe . server_recv (& mut buf [.. len]) , Ok (1200)) ; let frames = [frame :: Frame :: Padding { len : 10 }] ; let written = test_utils :: encode_pkt (& mut pipe . server , Type :: Initial , & frames , & mut buf ,) . unwrap () ; buf [written - 1] = ! buf [written - 1] ; assert_eq ! (pipe . client_recv (& mut buf [.. written]) , Ok (71)) ; assert ! (! pipe . client . is_closed ()) ; assert ! (pipe . client . idle_timer . is_some ()) ; }
};
}
