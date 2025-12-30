// Generated macro for test_file_write_read (function)
macro_rules! Depcrate_tests_fstest_file_write_read {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_write_read"}
// Dependencies: {}
pub fn test_file_write_read < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Write :: CODE) ; test . probe . is_supported (opcode :: Read :: CODE) ;) ; println ! ("test file_write_read") ; let fd = tempfile :: tempfile () ? ; let fd = types :: Fd (fd . as_raw_fd ()) ; utils :: write_read (ring , fd , fd) ? ; Ok (()) }
};
}
