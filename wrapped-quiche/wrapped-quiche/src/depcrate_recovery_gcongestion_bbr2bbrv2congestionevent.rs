// Generated macro for BBRv2CongestionEvent (struct)
macro_rules! Depcrate_recovery_gcongestion_bbr2BBRv2CongestionEvent {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"BBRv2CongestionEvent"}
// Dependencies: {}
struct BBRv2CongestionEvent { event_time : Instant , # [doc = " The congestion window prior to the processing of the ack/loss events."] prior_cwnd : usize , # [doc = " Total bytes inflight before the processing of the ack/loss events."] prior_bytes_in_flight : usize , # [doc = " Total bytes inflight after the processing of the ack/loss events."] bytes_in_flight : usize , # [doc = " Total bytes acked from acks in this event."] bytes_acked : usize , # [doc = " Total bytes lost from losses in this event."] bytes_lost : usize , # [doc = " Whether acked_packets indicates the end of a round trip."] end_of_round_trip : bool , is_probing_for_bandwidth : bool , sample_max_bandwidth : Option < Bandwidth > , # [doc = " Minimum rtt of all bandwidth samples from acked_packets."] # [doc = " None if acked_packets is empty."] sample_min_rtt : Option < Duration > , # [doc = " The send state of the largest packet in acked_packets, unless it is"] # [doc = " empty. If acked_packets is empty, it's the send state of the largest"] # [doc = " packet in lost_packets."] last_packet_send_state : SendTimeState , }
};
}
