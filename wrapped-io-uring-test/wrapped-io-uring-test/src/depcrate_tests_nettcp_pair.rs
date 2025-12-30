// Generated macro for tcp_pair (function)
macro_rules! Depcrate_tests_nettcp_pair {
() => {
// Module: crate::tests::net
// Provides: {"tcp_pair"}
// Dependencies: {}
fn tcp_pair () -> io :: Result < (TcpStream , TcpStream) > { let listener = TCP_LISTENER . get_or_try_init (| | TcpListener :: bind ("127.0.0.1:0")) ? ; let addr = listener . local_addr () ? ; let send_stream = TcpStream :: connect (addr) ? ; let (recv_stream , _) = listener . accept () ? ; Ok ((send_stream , recv_stream)) }
};
}
