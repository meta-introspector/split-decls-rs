// Generated macro for test_timeout_count (function)
macro_rules! Depcrate_tests_timeouttest_timeout_count {
() => {
// Module: crate::tests::timeout
// Provides: {"test_timeout_count"}
// Dependencies: {}
pub fn test_timeout_count < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Timeout :: CODE) ;) ; println ! ("test timeout_count") ; let ts = types :: Timespec :: new () . sec (1) ; let timeout_e = opcode :: Timeout :: new (& ts) . count (1) ; let nop_e = opcode :: Nop :: new () ; unsafe { let mut queue = ring . submission () ; queue . push (& timeout_e . build () . user_data (0x0c) . into ()) . expect ("queue is full") ; queue . push (& nop_e . build () . user_data (0x0d) . into ()) . expect ("queue is full") ; } let start = Instant :: now () ; ring . submit_and_wait (2) ? ; assert_eq ! (start . elapsed () . as_secs () , 0) ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_by_key (| cqe | cqe . user_data ()) ; assert_eq ! (cqes . len () , 2) ; assert_eq ! (cqes [0] . user_data () , 0x0c) ; assert_eq ! (cqes [1] . user_data () , 0x0d) ; assert_eq ! (cqes [0] . result () , 0) ; assert_eq ! (cqes [1] . result () , 0) ; Ok (()) }
};
}
