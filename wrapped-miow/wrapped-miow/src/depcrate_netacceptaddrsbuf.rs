// Generated macro for AcceptAddrsBuf (struct)
macro_rules! Depcrate_netAcceptAddrsBuf {
() => {
// Module: crate::net
// Provides: {"AcceptAddrsBuf"}
// Dependencies: {}
# [doc = " A type to represent a buffer in which an accepted socket's address will be"] # [doc = " stored."] # [doc = ""] # [doc = " This type is used with the `accept_overlapped` method on the"] # [doc = " `TcpListenerExt` trait to provide space for the overlapped I/O operation to"] # [doc = " fill in the socket addresses upon completion."] # [repr (C)] pub struct AcceptAddrsBuf { local : SOCKADDR_STORAGE , _pad1 : [u8 ; 16] , remote : SOCKADDR_STORAGE , _pad2 : [u8 ; 16] , }
};
}
