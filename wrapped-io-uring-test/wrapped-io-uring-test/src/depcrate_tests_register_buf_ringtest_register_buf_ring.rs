// Generated macro for test_register_buf_ring (function)
macro_rules! Depcrate_tests_register_buf_ringtest_register_buf_ring {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"test_register_buf_ring"}
// Dependencies: {}
pub fn test_register_buf_ring < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> io :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: UringCmd16 :: CODE) ;) ; println ! ("test register_buf_ring") ; buf_ring_reg_and_unreg (ring , test) ? ; buf_ring_play (ring , test) ? ; Ok (()) }
};
}
