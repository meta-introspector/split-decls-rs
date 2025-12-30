// Generated macro for test_debug_print (function)
macro_rules! Depcrate_tests_queuetest_debug_print {
() => {
// Module: crate::tests::queue
// Provides: {"test_debug_print"}
// Dependencies: {}
pub fn test_debug_print < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! { test ; } println ! ("test debug_print") ; let mut sq = ring . submission () ; let num_to_sub = sq . capacity () ; for _ in 0 .. num_to_sub { unsafe { sq . push (& opcode :: Nop :: new () . build () . user_data (0x42) . into ()) . expect ("queue is full") ; } } println ! ("Full: {:?}" , sq) ; drop (sq) ; ring . submit_and_wait (num_to_sub) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , num_to_sub) ; for cqe in cqes { assert_eq ! (cqe . user_data () , 0x42) ; assert_eq ! (cqe . result () , 0) ; } println ! ("Empty: {:?}" , ring . submission ()) ; Ok (()) }
};
}
