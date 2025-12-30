// Generated macro for connect_blocking (function)
macro_rules! Depcrate_addressconnect_blocking {
() => {
// Module: crate::address
// Provides: {"connect_blocking"}
// Dependencies: {}
pub fn connect_blocking (addr : & str) -> Result < UnixStream , Box < dyn std :: error :: Error > > { let sockaddr = address_to_sockaddr_un (addr) ? ; crate :: sys :: connect_blocking (& sockaddr) }
};
}
