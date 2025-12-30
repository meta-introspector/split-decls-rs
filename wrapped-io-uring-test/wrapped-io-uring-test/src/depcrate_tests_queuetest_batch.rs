// Generated macro for test_batch (function)
macro_rules! Depcrate_tests_queuetest_batch {
() => {
// Module: crate::tests::queue
// Provides: {"test_batch"}
// Dependencies: {}
pub fn test_batch < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { use std :: mem :: MaybeUninit ; require ! { test ; } println ! ("test batch") ; assert ! (ring . completion () . is_empty ()) ; unsafe { let sqes = vec ! [opcode :: Nop :: new () . build () . user_data (0x09) . into () ; 5] ; let mut sq = ring . submission () ; assert_eq ! (sq . capacity () , 8) ; sq . push_multiple (& sqes) . unwrap () ; assert_eq ! (sq . len () , 5) ; let ret = sq . push_multiple (& sqes) ; assert ! (ret . is_err ()) ; assert_eq ! (sq . len () , 5) ; sq . push_multiple (& sqes [.. 3]) . unwrap () ; } ring . submit_and_wait (8) ? ; let mut cqes = (0 .. 10) . map (| _ | MaybeUninit :: uninit ()) . collect :: < Vec < _ > > () ; let cqes = ring . completion () . fill (& mut cqes) ; assert_eq ! (cqes . len () , 8) ; for entry in cqes { let entry : cqueue :: Entry = entry . clone () . into () ; assert_eq ! (entry . user_data () , 0x09) ; } Ok (()) }
};
}
