// Generated macro for test_file_fallocate (function)
macro_rules! Depcrate_tests_fstest_file_fallocate {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_fallocate"}
// Dependencies: {}
pub fn test_file_fallocate < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Fallocate :: CODE) ;) ; println ! ("test file_fallocate") ; let fd = tempfile :: tempfile () ? ; let fd = types :: Fd (fd . as_raw_fd ()) ; let falloc_e = opcode :: Fallocate :: new (fd , 1024) ; unsafe { ring . submission () . push (& falloc_e . build () . user_data (0x10) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x10) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
