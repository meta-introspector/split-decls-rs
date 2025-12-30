// Generated macro for test_timeout_remove (function)
macro_rules! Depcrate_tests_timeouttest_timeout_remove {
() => {
// Module: crate::tests::timeout
// Provides: {"test_timeout_remove"}
// Dependencies: {}
pub fn test_timeout_remove < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Timeout :: CODE) ; test . probe . is_supported (opcode :: TimeoutRemove :: CODE) ;) ; println ! ("test timeout_remove") ; let ts = types :: Timespec :: new () . sec (1) ; let timeout_e = opcode :: Timeout :: new (& ts) ; unsafe { let mut queue = ring . submission () ; queue . push (& timeout_e . build () . user_data (0x10) . into ()) . expect ("queue is full") ; } ring . submit () ? ; let timeout_e = opcode :: TimeoutRemove :: new (0x10) ; unsafe { let mut queue = ring . submission () ; queue . push (& timeout_e . build () . user_data (0x11) . into ()) . expect ("queue is full") ; } let start = Instant :: now () ; ring . submit_and_wait (2) ? ; assert_eq ! (start . elapsed () . as_secs () , 0) ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_by_key (| cqe | cqe . user_data ()) ; assert_eq ! (cqes . len () , 2) ; assert_eq ! (cqes [0] . user_data () , 0x10) ; assert_eq ! (cqes [1] . user_data () , 0x11) ; assert_eq ! (cqes [0] . result () , - libc :: ECANCELED) ; assert_eq ! (cqes [1] . result () , 0) ; Ok (()) }
};
}
