// Generated macro for min_payload (function)
macro_rules! Depcrate_testsmin_payload {
() => {
// Module: crate::tests
// Provides: {"min_payload"}
// Dependencies: {}
# [rstest] fn min_payload (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; let frames = [frame :: Frame :: Padding { len : 4 }] ; let pkt_type = Type :: Initial ; let written = test_utils :: encode_pkt (& mut pipe . client , pkt_type , & frames , & mut buf) . unwrap () ; assert_eq ! (pipe . server_recv (& mut buf [.. written]) , Ok (written)) ; let initial_path = pipe . server . paths . get_active () . expect ("initial path not found") ; assert_eq ! (initial_path . max_send_bytes , 195) ; pipe . server . paths . get_active_mut () . expect ("no active path") . recovery . inc_loss_probes (packet :: Epoch :: Initial) ; let initial_path = pipe . server . paths . get_active_mut () . expect ("initial path not found") ; initial_path . max_send_bytes = 60 ; assert_eq ! (pipe . server . send (& mut buf) , Err (Error :: Done)) ; }
};
}
