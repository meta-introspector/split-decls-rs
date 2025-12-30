// Generated macro for get_ephemeral_port (function)
macro_rules! Depcrate_fd_socket_tcpget_ephemeral_port {
() => {
// Module: crate::fd::socket::tcp
// Provides: {"get_ephemeral_port"}
// Dependencies: {}
fn get_ephemeral_port () -> u16 { static LOCAL_ENDPOINT : AtomicU16 = AtomicU16 :: new (49152) ; LOCAL_ENDPOINT . fetch_add (1 , Ordering :: SeqCst) }
};
}
