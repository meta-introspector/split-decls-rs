// Generated macro for test_tcp_shutdown (function)
macro_rules! Depcrate_tests_nettest_tcp_shutdown {
() => {
// Module: crate::tests::net
// Provides: {"test_tcp_shutdown"}
// Dependencies: {}
pub fn test_tcp_shutdown < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Write :: CODE) ; test . probe . is_supported (opcode :: Shutdown :: CODE) ;) ; println ! ("test tcp_shutdown") ; const SHUT_WR : i32 = 1 ; let listener = TCP_LISTENER . get_or_try_init (| | TcpListener :: bind ("127.0.0.1:0")) ? ; let sock_fd = types :: Fd (listener . as_raw_fd ()) ; let shutdown_e = opcode :: Shutdown :: new (sock_fd , SHUT_WR) ; unsafe { ring . submission () . push (& shutdown_e . build () . user_data (0x28) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x28) ; assert_eq ! (cqes [0] . result () , 0) ; let text = b"C'est la vie" ; let write_e = opcode :: Write :: new (sock_fd , text . as_ptr () , text . len () as _) ; unsafe { ring . submission () . push (& write_e . build () . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . result () , - 32) ; Ok (()) }
};
}
