// Generated macro for client_connect (function)
macro_rules! Depcrate_connectionclient_connect {
() => {
// Module: crate::connection
// Provides: {"client_connect"}
// Dependencies: {}
pub fn client_connect < A : ToSocketAddrs > (addr : A) -> io :: Result < TcpStream > { TcpStream :: connect (addr) }
};
}
