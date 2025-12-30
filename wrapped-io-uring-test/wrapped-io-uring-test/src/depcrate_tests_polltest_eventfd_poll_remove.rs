// Generated macro for test_eventfd_poll_remove (function)
macro_rules! Depcrate_tests_polltest_eventfd_poll_remove {
() => {
// Module: crate::tests::poll
// Provides: {"test_eventfd_poll_remove"}
// Dependencies: {}
pub fn test_eventfd_poll_remove < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: PollAdd :: CODE) ; test . probe . is_supported (opcode :: PollRemove :: CODE) ;) ; println ! ("test eventfd_poll_remove") ; let mut fd = unsafe { let fd = libc :: eventfd (0 , libc :: EFD_CLOEXEC) ; if fd == - 1 { return Err (io :: Error :: last_os_error () . into ()) ; } File :: from_raw_fd (fd) } ; let poll_e = opcode :: PollAdd :: new (types :: Fd (fd . as_raw_fd ()) , libc :: POLLIN as _) ; unsafe { let mut queue = ring . submission () ; queue . push (& poll_e . build () . user_data (0x05) . into ()) . expect ("queue is full") ; } ring . submit () ? ; let poll_e = opcode :: PollRemove :: new (0x05) ; unsafe { let mut queue = ring . submission () ; queue . push (& poll_e . build () . user_data (0x06) . into ()) . expect ("queue is full") ; } ring . submit () ? ; thread :: sleep (Duration :: from_millis (200)) ; fd . write_all (& 0x1u64 . to_ne_bytes ()) ? ; ring . submit_and_wait (2) ? ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_by_key (| cqe | cqe . user_data ()) ; assert_eq ! (cqes . len () , 2) ; assert_eq ! (cqes [0] . user_data () , 0x05) ; assert_eq ! (cqes [1] . user_data () , 0x06) ; assert_eq ! (cqes [0] . result () , - libc :: ECANCELED) ; assert_eq ! (cqes [1] . result () , 0) ; Ok (()) }
};
}
