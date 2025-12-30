// Generated macro for test_queue_split (function)
macro_rules! Depcrate_tests_queuetest_queue_split {
() => {
// Module: crate::tests::queue
// Provides: {"test_queue_split"}
// Dependencies: {}
pub fn test_queue_split < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! { test ; } println ! ("test queue_split") ; let (submitter , mut sq , mut cq) = ring . split () ; assert ! (sq . is_empty ()) ; for _ in 0 .. sq . capacity () { unsafe { sq . push (& opcode :: Nop :: new () . build () . into ()) . expect ("queue is full") ; } } assert ! (sq . is_full ()) ; sq . sync () ; assert_eq ! (submitter . submit () ?, sq . capacity ()) ; assert ! (sq . is_full ()) ; sq . sync () ; assert ! (sq . is_empty ()) ; assert ! (cq . is_empty ()) ; cq . sync () ; assert_eq ! (cq . len () , sq . capacity ()) ; assert_eq ! (cq . by_ref () . count () , sq . capacity ()) ; cq . sync () ; Ok (()) }
};
}
