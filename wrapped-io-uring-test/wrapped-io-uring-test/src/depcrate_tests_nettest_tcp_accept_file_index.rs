// Generated macro for test_tcp_accept_file_index (function)
macro_rules! Depcrate_tests_nettest_tcp_accept_file_index {
() => {
// Module: crate::tests::net
// Provides: {"test_tcp_accept_file_index"}
// Dependencies: {}
pub fn test_tcp_accept_file_index < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Accept :: CODE) ; test . probe . is_supported (opcode :: Socket :: CODE) ;) ; println ! ("test tcp_accept_file_index") ; let _ = ring . submitter () . unregister_files () ; ring . submitter () . register_files_sparse (5) . unwrap () ; let listener = TCP_LISTENER . get_or_try_init (| | TcpListener :: bind ("127.0.0.1:0")) ? ; let addr = listener . local_addr () ? ; let fd = types :: Fd (listener . as_raw_fd ()) ; let _stream = TcpStream :: connect (addr) ? ; let mut sockaddr : libc :: sockaddr = unsafe { mem :: zeroed () } ; let mut addrlen : libc :: socklen_t = mem :: size_of :: < libc :: sockaddr > () as _ ; let dest_slot = types :: DestinationSlot :: try_from_slot_target (4) . unwrap () ; let accept_e = opcode :: Accept :: new (fd , & mut sockaddr , & mut addrlen) ; let accept_e = accept_e . file_index (Some (dest_slot)) ; unsafe { ring . submission () . push (& accept_e . build () . user_data (0x0e) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x0e) ; assert_eq ! (cqes [0] . result () , 0) ; ring . submitter () . unregister_files () . unwrap () ; Ok (()) }
};
}
