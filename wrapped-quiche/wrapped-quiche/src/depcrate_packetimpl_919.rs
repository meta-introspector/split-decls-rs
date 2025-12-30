// Generated macro for impl_919 (impl)
macro_rules! Depcrate_packetimpl_919 {
() => {
// Module: crate::packet
// Provides: {"impl_919"}
// Dependencies: {}
impl PktNumSpace { pub fn new () -> PktNumSpace { PktNumSpace { largest_rx_pkt_num : 0 , largest_rx_pkt_time : Instant :: now () , largest_rx_non_probing_pkt_num : 0 , largest_tx_pkt_num : None , recv_pkt_need_ack : ranges :: RangeSet :: new (crate :: MAX_ACK_RANGES) , recv_pkt_num : PktNumWindow :: default () , ack_elicited : false , } } pub fn clear (& mut self) { self . ack_elicited = false ; } pub fn ready (& self) -> bool { self . ack_elicited } pub fn on_packet_sent (& mut self , sent_pkt : & recovery :: Sent) { self . largest_tx_pkt_num = self . largest_tx_pkt_num . max (Some (sent_pkt . pkt_num)) ; } }
};
}
