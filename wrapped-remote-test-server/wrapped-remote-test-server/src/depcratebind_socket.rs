// Generated macro for bind_socket (function)
macro_rules! Depcratebind_socket {
() => {
// Module: crate
// Provides: {"bind_socket"}
// Dependencies: {}
fn bind_socket (addr : SocketAddr) -> TcpListener { for _ in 0 .. (NUMBER_OF_RETRIES - 1) { if let Ok (x) = TcpListener :: bind (addr) { return x ; } std :: thread :: sleep (std :: time :: Duration :: from_secs (RETRY_INTERVAL)) ; } TcpListener :: bind (addr) . unwrap () }
};
}
