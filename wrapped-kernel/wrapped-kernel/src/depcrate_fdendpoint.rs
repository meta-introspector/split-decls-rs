// Generated macro for Endpoint (enum)
macro_rules! Depcrate_fdEndpoint {
() => {
// Module: crate::fd
// Provides: {"Endpoint"}
// Dependencies: {}
# [cfg (any (feature = "net" , feature = "vsock"))] # [derive (Debug)] pub (crate) enum Endpoint { # [cfg (feature = "net")] Ip (IpEndpoint) , # [cfg (feature = "vsock")] Vsock (socket :: vsock :: VsockEndpoint) , }
};
}
