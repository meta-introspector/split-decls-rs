// Generated macro for test_nop (function)
macro_rules! Depcrate_tests_queuetest_nop {
() => {
// Module: crate::tests::queue
// Provides: {"test_nop"}
// Dependencies: {}
pub fn test_nop < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! { test ; } println ! ("test nop") ; let nop_e = opcode :: Nop :: new () . build () . user_data (0x42) . into () ; unsafe { let mut queue = ring . submission () ; queue . push (& nop_e) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x42) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
