// Generated macro for AcceptAddrs (struct)
macro_rules! Depcrate_netAcceptAddrs {
() => {
// Module: crate::net
// Provides: {"AcceptAddrs"}
// Dependencies: {}
# [doc = " The parsed return value of `AcceptAddrsBuf`."] pub struct AcceptAddrs < 'a > { local : * mut SOCKADDR , local_len : i32 , remote : * mut SOCKADDR , remote_len : i32 , _data : & 'a AcceptAddrsBuf , }
};
}
