// Generated macro for test_timeout_multishot (function)
macro_rules! Depcrate_tests_timeouttest_timeout_multishot {
() => {
// Module: crate::tests::timeout
// Provides: {"test_timeout_multishot"}
// Dependencies: {}
pub fn test_timeout_multishot < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Timeout :: CODE) ;) ; println ! ("test timeout_multishot") ; let ts = types :: Timespec :: new () . sec (1) ; let timeout_e = opcode :: Timeout :: new (& ts) . flags (types :: TimeoutFlags :: MULTISHOT) . count (2) ; unsafe { let mut queue = ring . submission () ; queue . push (& timeout_e . build () . user_data (0x0c) . into ()) . expect ("queue is full") ; } let start = Instant :: now () ; ring . submit_and_wait (1) ? ; assert_eq ! (start . elapsed () . as_secs () , 1) ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_by_key (| cqe | cqe . user_data ()) ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x0c) ; assert_eq ! (cqes [0] . result () , - libc :: ETIME) ; assert ! (cqueue :: more (cqes [0] . flags ())) ; ring . submit_and_wait (1) ? ; assert_eq ! (start . elapsed () . as_secs () , 2) ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_by_key (| cqe | cqe . user_data ()) ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x0c) ; assert_eq ! (cqes [0] . result () , - libc :: ETIME) ; assert ! (! cqueue :: more (cqes [0] . flags ())) ; Ok (()) }
};
}
