// Generated macro for ProtectedHeader (enum)
macro_rules! Depcrate_packetProtectedHeader {
() => {
// Module: crate::packet
// Provides: {"ProtectedHeader"}
// Dependencies: {}
# [doc = " Plain packet header"] # [derive (Clone , Debug)] pub enum ProtectedHeader { # [doc = " An Initial packet header"] Initial (ProtectedInitialHeader) , # [doc = " A Long packet header, as used during the handshake"] Long { # [doc = " Type of the Long header packet"] ty : LongType , # [doc = " Destination Connection ID"] dst_cid : ConnectionId , # [doc = " Source Connection ID"] src_cid : ConnectionId , # [doc = " Length of the packet payload"] len : u64 , # [doc = " QUIC version"] version : u32 , } , # [doc = " A Retry packet header"] Retry { # [doc = " Destination Connection ID"] dst_cid : ConnectionId , # [doc = " Source Connection ID"] src_cid : ConnectionId , # [doc = " QUIC version"] version : u32 , } , # [doc = " A short packet header, as used during the data phase"] Short { # [doc = " Spin bit"] spin : bool , # [doc = " Destination Connection ID"] dst_cid : ConnectionId , } , # [doc = " A Version Negotiation packet header"] VersionNegotiate { # [doc = " Random value"] random : u8 , # [doc = " Destination Connection ID"] dst_cid : ConnectionId , # [doc = " Source Connection ID"] src_cid : ConnectionId , } , }
};
}
