// Generated macro for test_timeout_abs (function)
macro_rules! Depcrate_tests_timeouttest_timeout_abs {
() => {
// Module: crate::tests::timeout
// Provides: {"test_timeout_abs"}
// Dependencies: {}
pub fn test_timeout_abs < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Timeout :: CODE) ;) ; println ! ("test timeout_abs") ; let mut now = libc :: timespec { tv_sec : 0 , tv_nsec : 0 , } ; let ret = unsafe { libc :: clock_gettime (libc :: CLOCK_MONOTONIC , & mut now) } ; assert_eq ! (ret , 0) ; let ts = types :: Timespec :: new () . sec (now . tv_sec as u64 + 2) . nsec (now . tv_nsec as u32) ; let timeout_e = opcode :: Timeout :: new (& ts) . flags (types :: TimeoutFlags :: ABS) ; unsafe { let mut queue = ring . submission () ; queue . push (& timeout_e . build () . user_data (0x19) . into ()) . expect ("queue is full") ; } let start = Instant :: now () ; ring . submit_and_wait (1) ? ; assert ! (start . elapsed () . as_secs () >= 1) ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x19) ; assert_eq ! (cqes [0] . result () , - libc :: ETIME) ; Ok (()) }
};
}
