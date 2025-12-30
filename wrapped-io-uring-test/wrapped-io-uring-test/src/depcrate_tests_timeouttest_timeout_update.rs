// Generated macro for test_timeout_update (function)
macro_rules! Depcrate_tests_timeouttest_timeout_update {
() => {
// Module: crate::tests::timeout
// Provides: {"test_timeout_update"}
// Dependencies: {}
pub fn test_timeout_update < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Timeout :: CODE) ; test . probe . is_supported (opcode :: TimeoutRemove :: CODE) ;) ; println ! ("test timeout_update") ; let ts = types :: Timespec :: new () . nsec (1_000_000) ; let timeout_e = opcode :: Timeout :: new (& ts) ; unsafe { let mut queue = ring . submission () ; queue . push (& timeout_e . build () . user_data (0x10) . into ()) . expect ("queue is full") ; } ring . submit () ? ; let ts = types :: Timespec :: new () . nsec (2_000_000) ; let timeout_e = opcode :: TimeoutUpdate :: new (0x10 , & ts) ; unsafe { let mut queue = ring . submission () ; queue . push (& timeout_e . build () . user_data (0x11) . into ()) . expect ("queue is full") ; } let start = Instant :: now () ; ring . submit_and_wait (2) ? ; assert ! (start . elapsed () . as_nanos () >= 2_000_000) ; let mut cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; cqes . sort_by_key (| cqe | cqe . user_data ()) ; assert_eq ! (cqes . len () , 2) ; assert_eq ! (cqes [0] . user_data () , 0x10) ; assert_eq ! (cqes [1] . user_data () , 0x11) ; assert_eq ! (cqes [0] . result () , - libc :: ETIME) ; assert_eq ! (cqes [1] . result () , 0) ; Ok (()) }
};
}
