// Generated macro for SentPacket (struct)
macro_rules! Depcrate_connection_spacesSentPacket {
() => {
// Module: crate::connection::spaces
// Provides: {"SentPacket"}
// Dependencies: {}
# [doc = " Represents one or more packets subject to retransmission"] # [derive (Debug , Clone)] pub (super) struct SentPacket { # [doc = " [`PathData::generation`](super::PathData::generation) of the path on which this packet was sent"] pub (super) path_generation : u64 , # [doc = " The time the packet was sent."] pub (super) time_sent : Instant , # [doc = " The number of bytes sent in the packet, not including UDP or IP overhead, but including QUIC"] # [doc = " framing overhead. Zero if this packet is not counted towards congestion control, i.e. not an"] # [doc = " \"in flight\" packet."] pub (super) size : u16 , # [doc = " Whether an acknowledgement is expected directly in response to this packet."] pub (super) ack_eliciting : bool , # [doc = " The largest packet number acknowledged by this packet"] pub (super) largest_acked : Option < u64 > , # [doc = " Data which needs to be retransmitted in case the packet is lost."] # [doc = " The data is boxed to minimize `SentPacket` size for the typical case of"] # [doc = " packets only containing ACKs and STREAM frames."] pub (super) retransmits : ThinRetransmits , # [doc = " Metadata for stream frames in a packet"] # [doc = ""] # [doc = " The actual application data is stored with the stream state."] pub (super) stream_frames : frame :: StreamMetaVec , }
};
}
