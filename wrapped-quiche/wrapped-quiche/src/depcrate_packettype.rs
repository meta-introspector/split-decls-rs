// Generated macro for Type (enum)
macro_rules! Depcrate_packetType {
() => {
// Module: crate::packet
// Provides: {"Type"}
// Dependencies: {}
# [doc = " QUIC packet type."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Type { # [doc = " Initial packet."] Initial , # [doc = " Retry packet."] Retry , # [doc = " Handshake packet."] Handshake , # [doc = " 0-RTT packet."] ZeroRTT , # [doc = " Version negotiation packet."] VersionNegotiation , # [doc = " 1-RTT short header packet."] Short , }
};
}
