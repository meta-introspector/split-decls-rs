// Generated macro for SendInfo (struct)
macro_rules! DepcrateSendInfo {
() => {
// Module: crate
// Provides: {"SendInfo"}
// Dependencies: {}
# [doc = " Ancillary information about outgoing packets."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct SendInfo { # [doc = " The local address the packet should be sent from."] pub from : SocketAddr , # [doc = " The remote address the packet should be sent to."] pub to : SocketAddr , # [doc = " The time to send the packet out."] # [doc = ""] # [doc = " See [Pacing] for more details."] # [doc = ""] # [doc = " [Pacing]: index.html#pacing"] pub at : Instant , }
};
}
