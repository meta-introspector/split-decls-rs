// Generated macro for invalid_packet (function)
macro_rules! Depcrate_testsinvalid_packet {
() => {
// Module: crate::tests
// Provides: {"invalid_packet"}
// Dependencies: {}
# [rstest] # [doc = " Tests that invalid packets don't cause the connection to be closed."] fn invalid_packet (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let frames = [frame :: Frame :: Padding { len : 10 }] ; let written = test_utils :: encode_pkt (& mut pipe . client , Type :: Short , & frames , & mut buf) . unwrap () ; buf [written - 1] = ! buf [written - 1] ; assert_eq ! (pipe . server_recv (& mut buf [.. written]) , Ok (written)) ; buf [0] = 255 ; assert_eq ! (pipe . server_recv (& mut buf [.. written]) , Ok (written)) ; }
};
}
