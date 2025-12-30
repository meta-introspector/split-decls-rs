// Generated macro for Header (enum)
macro_rules! Depcrate_packetHeader {
() => {
// Module: crate::packet
// Provides: {"Header"}
// Dependencies: {}
# [cfg_attr (test , derive (Clone))] # [derive (Debug)] pub (crate) enum Header { Initial (InitialHeader) , Long { ty : LongType , dst_cid : ConnectionId , src_cid : ConnectionId , number : PacketNumber , version : u32 , } , Retry { dst_cid : ConnectionId , src_cid : ConnectionId , version : u32 , } , Short { spin : bool , key_phase : bool , dst_cid : ConnectionId , number : PacketNumber , } , VersionNegotiate { random : u8 , src_cid : ConnectionId , dst_cid : ConnectionId , } , }
};
}
