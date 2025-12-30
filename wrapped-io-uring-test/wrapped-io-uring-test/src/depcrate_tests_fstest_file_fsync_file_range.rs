// Generated macro for test_file_fsync_file_range (function)
macro_rules! Depcrate_tests_fstest_file_fsync_file_range {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_fsync_file_range"}
// Dependencies: {}
pub fn test_file_fsync_file_range < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: SyncFileRange :: CODE) ;) ; println ! ("test file_fsync_file_range") ; let mut fd = tempfile :: tempfile () ? ; let n = fd . write (& [0x2 ; 3 * 1024]) ? ; assert_eq ! (n , 3 * 1024) ; let n = fd . write (& [0x3 ; 1024]) ? ; assert_eq ! (n , 1024) ; let fd = types :: Fd (fd . as_raw_fd ()) ; let fsync_e = opcode :: SyncFileRange :: new (fd , 1024) . offset (3 * 1024) ; unsafe { ring . submission () . push (& fsync_e . build () . user_data (0x04) . into ()) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (cqes . len () , 1) ; assert_eq ! (cqes [0] . user_data () , 0x04) ; assert_eq ! (cqes [0] . result () , 0) ; Ok (()) }
};
}
