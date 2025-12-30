// Generated macro for PacketDecodeError (enum)
macro_rules! Depcrate_packetPacketDecodeError {
() => {
// Module: crate::packet
// Provides: {"PacketDecodeError"}
// Dependencies: {}
# [doc = " Packet decode error"] # [derive (Debug , Error , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum PacketDecodeError { # [doc = " Packet uses a QUIC version that is not supported"] # [error ("unsupported version {version:x}")] UnsupportedVersion { # [doc = " Source Connection ID"] src_cid : ConnectionId , # [doc = " Destination Connection ID"] dst_cid : ConnectionId , # [doc = " The version that was unsupported"] version : u32 , } , # [doc = " The packet header is invalid"] # [error ("invalid header: {0}")] InvalidHeader (& 'static str) , }
};
}
