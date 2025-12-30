// Generated macro for test_file_close (function)
macro_rules! Depcrate_tests_fstest_file_close {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_close"}
// Dependencies: {}
pub fn test_file_close < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Close :: CODE) ;) ; println ! ("test file_cloes") ; let fd = tempfile :: tempfile () ? ; let fd = types :: Fd (fd . into_raw_fd ()) ; let close_e = opcode :: Close :: new (fd) ; unsafe { ring . submission () . push (& close_e . build () . user_data (0x12) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x12) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
