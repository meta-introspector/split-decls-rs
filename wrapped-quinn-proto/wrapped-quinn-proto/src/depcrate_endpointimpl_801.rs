// Generated macro for impl_801 (impl)
macro_rules! Depcrate_endpointimpl_801 {
() => {
// Module: crate::endpoint
// Provides: {"impl_801"}
// Dependencies: {}
impl Incoming { # [doc = " The local IP address which was used when the peer established the connection"] # [doc = ""] # [doc = " This has the same behavior as [`Connection::local_ip`]."] pub fn local_ip (& self) -> Option < IpAddr > { self . addresses . local_ip } # [doc = " The peer's UDP address"] pub fn remote_address (& self) -> SocketAddr { self . addresses . remote } # [doc = " Whether the socket address that is initiating this connection has been validated"] # [doc = ""] # [doc = " This means that the sender of the initial packet has proved that they can receive traffic"] # [doc = " sent to `self.remote_address()`."] # [doc = ""] # [doc = " If `self.remote_address_validated()` is false, `self.may_retry()` is guaranteed to be true."] # [doc = " The inverse is not guaranteed."] pub fn remote_address_validated (& self) -> bool { self . token . validated } # [doc = " Whether it is legal to respond with a retry packet"] # [doc = ""] # [doc = " If `self.remote_address_validated()` is false, `self.may_retry()` is guaranteed to be true."] # [doc = " The inverse is not guaranteed."] pub fn may_retry (& self) -> bool { self . token . retry_src_cid . is_none () } # [doc = " The original destination connection ID sent by the client"] pub fn orig_dst_cid (& self) -> ConnectionId { self . token . orig_dst_cid } }
};
}
