// Generated macro for test_file_writev_readv (function)
macro_rules! Depcrate_tests_fstest_file_writev_readv {
() => {
// Module: crate::tests::fs
// Provides: {"test_file_writev_readv"}
// Dependencies: {}
pub fn test_file_writev_readv < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Writev :: CODE) ; test . probe . is_supported (opcode :: Readv :: CODE) ;) ; println ! ("test file_writev_readv") ; let fd = tempfile :: tempfile () ? ; let fd = types :: Fd (fd . as_raw_fd ()) ; utils :: writev_readv (ring , fd , fd) ? ; Ok (()) }
};
}
