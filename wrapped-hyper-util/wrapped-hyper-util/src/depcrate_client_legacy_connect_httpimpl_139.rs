// Generated macro for impl_139 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_139 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_139"}
// Dependencies: {}
impl Connection for TcpStream { fn connected (& self) -> Connected { let connected = Connected :: new () ; if let (Ok (remote_addr) , Ok (local_addr)) = (self . peer_addr () , self . local_addr ()) { connected . extra (HttpInfo { remote_addr , local_addr , }) } else { connected } } }
};
}
