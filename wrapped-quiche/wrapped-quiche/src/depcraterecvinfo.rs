// Generated macro for RecvInfo (struct)
macro_rules! DepcrateRecvInfo {
() => {
// Module: crate
// Provides: {"RecvInfo"}
// Dependencies: {}
# [doc = " Ancillary information about incoming packets."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct RecvInfo { # [doc = " The remote address the packet was received from."] pub from : SocketAddr , # [doc = " The local address the packet was received on."] pub to : SocketAddr , }
};
}
