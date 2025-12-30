// Generated macro for RecoveryEpoch (struct)
macro_rules! Depcrate_recovery_congestion_recoveryRecoveryEpoch {
() => {
// Module: crate::recovery::congestion::recovery
// Provides: {"RecoveryEpoch"}
// Dependencies: {}
# [derive (Default)] struct RecoveryEpoch { # [doc = " The time the most recent ack-eliciting packet was sent."] time_of_last_ack_eliciting_packet : Option < Instant > , # [doc = " The largest packet number acknowledged in the packet number space so"] # [doc = " far."] largest_acked_packet : Option < u64 > , # [doc = " The time at which the next packet in that packet number space can be"] # [doc = " considered lost based on exceeding the reordering window in time."] loss_time : Option < Instant > , # [doc = " An association of packet numbers in a packet number space to information"] # [doc = " about them."] sent_packets : VecDeque < Sent > , loss_probes : usize , in_flight_count : usize , acked_frames : Vec < frame :: Frame > , lost_frames : Vec < frame :: Frame > , # [doc = " The largest packet number sent in the packet number space so far."] # [cfg (test)] test_largest_sent_pkt_num_on_path : Option < u64 > , }
};
}
