// Generated macro for MAX_HANDSHAKE_OR_0RTT_HEADER_SIZE (const)
macro_rules! Depcrate_connectionMAX_HANDSHAKE_OR_0RTT_HEADER_SIZE {
() => {
// Module: crate::connection
// Provides: {"MAX_HANDSHAKE_OR_0RTT_HEADER_SIZE"}
// Dependencies: {}
# [doc = " Largest amount of space that could be occupied by a Handshake or 0-RTT packet's header"] # [doc = ""] # [doc = " Excludes packet-type-specific fields such as packet number or Initial token"] const MAX_HANDSHAKE_OR_0RTT_HEADER_SIZE : usize = 1 + 4 + 1 + MAX_CID_SIZE + 1 + MAX_CID_SIZE + VarInt :: from_u32 (u16 :: MAX as u32) . size () + 4 ;
};
}
