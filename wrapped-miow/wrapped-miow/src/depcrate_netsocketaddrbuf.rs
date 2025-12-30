// Generated macro for SocketAddrBuf (struct)
macro_rules! Depcrate_netSocketAddrBuf {
() => {
// Module: crate::net
// Provides: {"SocketAddrBuf"}
// Dependencies: {}
# [doc = " A type to represent a buffer in which a socket address will be stored."] # [doc = ""] # [doc = " This type is used with the `recv_from_overlapped` function on the"] # [doc = " `UdpSocketExt` trait to provide space for the overlapped I/O operation to"] # [doc = " fill in the address upon completion."] # [derive (Clone , Copy)] pub struct SocketAddrBuf { buf : SOCKADDR_STORAGE , len : i32 , }
};
}
