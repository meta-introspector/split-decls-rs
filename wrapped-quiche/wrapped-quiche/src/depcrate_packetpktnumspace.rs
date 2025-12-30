// Generated macro for PktNumSpace (struct)
macro_rules! Depcrate_packetPktNumSpace {
() => {
// Module: crate::packet
// Provides: {"PktNumSpace"}
// Dependencies: {}
pub struct PktNumSpace { # [doc = " The largest packet number received."] pub largest_rx_pkt_num : u64 , # [doc = " Time the largest packet number received."] pub largest_rx_pkt_time : Instant , # [doc = " The largest non-probing packet number."] pub largest_rx_non_probing_pkt_num : u64 , # [doc = " The largest packet number send in the packet number space so far."] pub largest_tx_pkt_num : Option < u64 > , # [doc = " Range of packet numbers that we need to send an ACK for."] pub recv_pkt_need_ack : ranges :: RangeSet , # [doc = " Tracks received packet numbers."] pub recv_pkt_num : PktNumWindow , # [doc = " Track if a received packet is ack eliciting."] pub ack_elicited : bool , }
};
}
