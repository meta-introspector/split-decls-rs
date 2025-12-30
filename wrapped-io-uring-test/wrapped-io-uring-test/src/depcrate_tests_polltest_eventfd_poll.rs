// Generated macro for test_eventfd_poll (function)
macro_rules! Depcrate_tests_polltest_eventfd_poll {
() => {
// Module: crate::tests::poll
// Provides: {"test_eventfd_poll"}
// Dependencies: {}
pub fn test_eventfd_poll < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: PollAdd :: CODE) ;) ; println ! ("test eventfd_poll") ; let mut fd = unsafe { let fd = libc :: eventfd (0 , libc :: EFD_CLOEXEC) ; if fd == - 1 { return Err (io :: Error :: last_os_error () . into ()) ; } File :: from_raw_fd (fd) } ; let poll_e = opcode :: PollAdd :: new (types :: Fd (fd . as_raw_fd ()) , libc :: POLLIN as _) ; unsafe { let mut queue = ring . submission () ; queue . push (& poll_e . build () . user_data (0x04) . into ()) . expect ("queue is full") ; } ring . submit () ? ; thread :: sleep (Duration :: from_millis (200)) ; assert_eq ! (ring . completion () . len () , 0) ; fd . write_all (& 0x1u64 . to_ne_bytes ()) ? ; ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x04) ; assert_eq ! (cqes [0] . result () , 1) ; Ok (()) }
};
}
