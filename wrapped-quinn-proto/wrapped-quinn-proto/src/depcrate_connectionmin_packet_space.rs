// Generated macro for MIN_PACKET_SPACE (const)
macro_rules! Depcrate_connectionMIN_PACKET_SPACE {
() => {
// Module: crate::connection
// Provides: {"MIN_PACKET_SPACE"}
// Dependencies: {}
# [doc = " Minimal remaining size to allow packet coalescing, excluding cryptographic tag"] # [doc = ""] # [doc = " This must be at least as large as the header for a well-formed empty packet to be coalesced,"] # [doc = " plus some space for frames. We only care about handshake headers because short header packets"] # [doc = " necessarily have smaller headers, and initial packets are only ever the first packet in a"] # [doc = " datagram (because we coalesce in ascending packet space order and the only reason to split a"] # [doc = " packet is when packet space changes)."] const MIN_PACKET_SPACE : usize = MAX_HANDSHAKE_OR_0RTT_HEADER_SIZE + 32 ;
};
}
