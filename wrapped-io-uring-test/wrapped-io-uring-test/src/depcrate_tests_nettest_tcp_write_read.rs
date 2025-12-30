// Generated macro for test_tcp_write_read (function)
macro_rules! Depcrate_tests_nettest_tcp_write_read {
() => {
// Module: crate::tests::net
// Provides: {"test_tcp_write_read"}
// Dependencies: {}
pub fn test_tcp_write_read < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { require ! (test ; test . probe . is_supported (opcode :: Write :: CODE) ; test . probe . is_supported (opcode :: Read :: CODE) ;) ; println ! ("test tcp_write_read") ; let (send_stream , recv_stream) = tcp_pair () ? ; let send_fd = types :: Fd (send_stream . as_raw_fd ()) ; let recv_fd = types :: Fd (recv_stream . as_raw_fd ()) ; utils :: write_read (ring , send_fd , recv_fd) ? ; Ok (()) }
};
}
