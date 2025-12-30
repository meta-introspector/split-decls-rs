// Generated macro for test_file_fsync (function)
macro_rules! Depcrate_tests_fstest_file_fsync {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_fsync"}
// Dependencies: {}
pub fn test_file_fsync < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Fsync :: CODE) ;) ; println ! ("test file_fsync") ; let mut fd = tempfile :: tempfile () ? ; let n = fd . write (& [0x1]) ? ; assert_eq ! (n , 1) ; let fd = types :: Fd (fd . as_raw_fd ()) ; let fsync_e = opcode :: Fsync :: new (fd) ; unsafe { ring . submission () . push (& fsync_e . build () . user_data (0x03) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x03) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
