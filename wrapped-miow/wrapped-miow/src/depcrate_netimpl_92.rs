// Generated macro for impl_92 (impl)
macro_rules! Depcrate_netimpl_92 {
() => {
// Module: crate::net
// Provides: {"impl_92"}
// Dependencies: {}
impl SocketAddrBuf { # [doc = " Creates a new blank socket address buffer."] # [doc = ""] # [doc = " This should be used before a call to `recv_from_overlapped` overlapped"] # [doc = " to create an instance to pass down."] pub fn new () -> SocketAddrBuf { SocketAddrBuf { buf : unsafe { mem :: zeroed () } , len : mem :: size_of :: < SOCKADDR_STORAGE > () as i32 , } } # [doc = " Parses this buffer to return a standard socket address."] # [doc = ""] # [doc = " This function should be called after the buffer has been filled in with"] # [doc = " a call to `recv_from_overlapped` being completed. It will interpret the"] # [doc = " address filled in and return the standard socket address type."] # [doc = ""] # [doc = " If an error is encountered then `None` is returned."] pub fn to_socket_addr (& self) -> Option < SocketAddr > { unsafe { ptrs_to_socket_addr (& self . buf as * const _ as * const _ , self . len) } } }
};
}
