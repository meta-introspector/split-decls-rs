// Generated macro for test_tcp_connect (function)
macro_rules! Depcrate_tests_nettest_tcp_connect {
() => {
// Module: crate::tests::net
// Provides: {"test_tcp_connect"}
// Dependencies: {}
pub fn test_tcp_connect < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { use socket2 :: { Domain , Protocol , SockAddr , Socket , Type } ; require ! (test ; test . probe . is_supported (opcode :: Connect :: CODE) ;) ; println ! ("test tcp_connect") ; let listener = TCP_LISTENER . get_or_try_init (| | TcpListener :: bind ("127.0.0.1:0")) ? ; let addr = listener . local_addr () ? ; let sockaddr = SockAddr :: from (addr) ; let stream = Socket :: new (Domain :: IPV4 , Type :: STREAM , Some (Protocol :: TCP)) ? ; let connect_e = opcode :: Connect :: new (types :: Fd (stream . as_raw_fd ()) , sockaddr . as_ptr () as * const _ , sockaddr . len () ,) ; unsafe { ring . submission () . push (& connect_e . build () . user_data (0x0f) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x0f) ; assert_eq ! (cqes [0] . result () , 0) ; let _ = listener . accept () ? ; Ok (()) }
};
}
