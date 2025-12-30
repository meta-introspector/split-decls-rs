// Generated macro for test_tcp_send_recv (function)
macro_rules! Depcrate_tests_nettest_tcp_send_recv {
() => {
// Module: crate::tests::net
// Provides: {"test_tcp_send_recv"}
// Dependencies: {}
pub fn test_tcp_send_recv < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Send :: CODE) ; test . probe . is_supported (opcode :: Recv :: CODE) ;) ; println ! ("test tcp_send_recv") ; let (send_stream , recv_stream) = tcp_pair () ? ; let send_fd = types :: Fd (send_stream . as_raw_fd ()) ; let recv_fd = types :: Fd (recv_stream . as_raw_fd ()) ; let text = b"The quick brown fox jumps over the lazy dog." ; let mut output = vec ! [0 ; text . len ()] ; let send_e = opcode :: Send :: new (send_fd , text . as_ptr () , text . len () as _) ; let recv_e = opcode :: Recv :: new (recv_fd , output . as_mut_ptr () , output . len () as _) ; unsafe { let mut queue = ring . submission () ; let send_e = send_e . build () . user_data (0x01) . flags (squeue :: Flags :: IO_LINK) . into () ; queue . push (& send_e) . expect ("queue is full") ; queue . push (& recv_e . build () . user_data (0x02) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (2) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 2) ; assert_eq ! (cqes [0] . user_data () , 0x01) ; assert_eq ! (cqes [1] . user_data () , 0x02) ; assert_eq ! (cqes [0] . result () , text . len () as i32) ; assert_eq ! (cqes [1] . result () , text . len () as i32) ; assert_eq ! (& output [.. cqes [1] . result () as usize] , text) ; Ok (()) }
};
}
