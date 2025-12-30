// Generated macro for ListenEndpoint (enum)
macro_rules! Depcrate_fdListenEndpoint {
() => {
// Module: crate::fd
// Provides: {"ListenEndpoint"}
// Dependencies: {}
# [cfg (any (feature = "net" , feature = "vsock"))] # [derive (Debug)] pub (crate) enum ListenEndpoint { # [cfg (feature = "net")] Ip (IpListenEndpoint) , # [cfg (feature = "vsock")] Vsock (socket :: vsock :: VsockListenEndpoint) , }
};
}
