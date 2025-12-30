// Generated macro for handshake_packet_type_corruption (function)
macro_rules! Depcrate_testshandshake_packet_type_corruption {
() => {
// Module: crate::tests
// Provides: {"handshake_packet_type_corruption"}
// Dependencies: {}
# [rstest] # [doc = " Tests that packets with corrupted type (from Handshake to Initial) are"] # [doc = " properly ignored."] fn handshake_packet_type_corruption (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; assert_eq ! (len , 1200) ; assert_eq ! (pipe . server_recv (& mut buf [.. len]) , Ok (len)) ; let flight = test_utils :: emit_flight (& mut pipe . server) . unwrap () ; test_utils :: process_flight (& mut pipe . client , flight) . unwrap () ; let active_pid = pipe . client . paths . get_active_path_id () . expect ("no active") ; let (ty , len) = pipe . client . send_single (& mut buf , active_pid , false , Instant :: now ()) . unwrap () ; assert_eq ! (ty , Type :: Initial) ; assert_eq ! (pipe . server_recv (& mut buf [.. len]) , Ok (len)) ; let (ty , len) = pipe . client . send_single (& mut buf , active_pid , false , Instant :: now ()) . unwrap () ; assert_eq ! (ty , Type :: Handshake) ; buf [0] &= ! (0x20) ; let hdr = Header :: from_slice (& mut buf [.. len] , 0) . unwrap () ; assert_eq ! (hdr . ty , Type :: Initial) ; assert_eq ! (pipe . server_recv (& mut buf [.. len]) , Ok (len)) ; }
};
}
