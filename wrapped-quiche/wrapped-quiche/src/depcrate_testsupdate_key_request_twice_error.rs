// Generated macro for update_key_request_twice_error (function)
macro_rules! Depcrate_testsupdate_key_request_twice_error {
() => {
// Module: crate::tests
// Provides: {"update_key_request_twice_error"}
// Dependencies: {}
# [rstest] fn update_key_request_twice_error (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . advance () , Ok (())) ; let frames = [frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf > :: from (b"hello" , 0 , false) , }] ; assert_eq ! (pipe . client_update_key () , Ok (())) ; let written = test_utils :: encode_pkt (& mut pipe . client , Type :: Short , & frames , & mut buf) . unwrap () ; assert_eq ! (pipe . server_recv (& mut buf [.. written]) , Ok (written)) ; assert_eq ! (pipe . client_update_key () , Ok (())) ; let written = test_utils :: encode_pkt (& mut pipe . client , Type :: Short , & frames , & mut buf) . unwrap () ; assert_eq ! (pipe . server_recv (& mut buf [.. written]) , Err (Error :: KeyUpdate)) ; }
};
}
