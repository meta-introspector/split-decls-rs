// Generated macro for test_tcp_writev_readv (function)
macro_rules! Depcrate_tests_nettest_tcp_writev_readv {
() => {
// Module: crate::tests::net
// Provides: {"test_tcp_writev_readv"}
// Dependencies: {}
pub fn test_tcp_writev_readv < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Writev :: CODE) ; test . probe . is_supported (opcode :: Readv :: CODE) ;) ; println ! ("test tcp_writev_readv") ; let (send_stream , recv_stream) = tcp_pair () ? ; let send_fd = types :: Fd (send_stream . as_raw_fd ()) ; let recv_fd = types :: Fd (recv_stream . as_raw_fd ()) ; utils :: writev_readv (ring , send_fd , recv_fd) ? ; Ok (()) }
};
}
