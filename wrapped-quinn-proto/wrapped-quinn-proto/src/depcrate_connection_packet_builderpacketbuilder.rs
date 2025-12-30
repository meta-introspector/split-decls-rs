// Generated macro for PacketBuilder (struct)
macro_rules! Depcrate_connection_packet_builderPacketBuilder {
() => {
// Module: crate::connection::packet_builder
// Provides: {"PacketBuilder"}
// Dependencies: {}
pub (super) struct PacketBuilder { pub (super) datagram_start : usize , pub (super) space : SpaceId , pub (super) partial_encode : PartialEncode , pub (super) ack_eliciting : bool , pub (super) exact_number : u64 , pub (super) short_header : bool , # [doc = " Smallest absolute position in the associated buffer that must be occupied by this packet's"] # [doc = " frames"] pub (super) min_size : usize , # [doc = " Largest absolute position in the associated buffer that may be occupied by this packet's"] # [doc = " frames"] pub (super) max_size : usize , pub (super) tag_len : usize , pub (super) _span : tracing :: span :: EnteredSpan , }
};
}
