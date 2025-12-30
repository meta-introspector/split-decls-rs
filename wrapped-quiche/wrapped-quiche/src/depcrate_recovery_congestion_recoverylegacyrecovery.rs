// Generated macro for LegacyRecovery (struct)
macro_rules! Depcrate_recovery_congestion_recoveryLegacyRecovery {
() => {
// Module: crate::recovery::congestion::recovery
// Provides: {"LegacyRecovery"}
// Dependencies: {}
pub struct LegacyRecovery { epochs : [RecoveryEpoch ; Epoch :: count ()] , loss_timer : LossDetectionTimer , pto_count : u32 , rtt_stats : RttStats , lost_spurious_count : usize , pkt_thresh : u64 , time_thresh : f64 , bytes_in_flight : BytesInFlight , bytes_sent : usize , bytes_lost : u64 , pub max_datagram_size : usize , # [cfg (feature = "qlog")] qlog_metrics : QlogMetrics , # [cfg (feature = "qlog")] qlog_prev_cc_state : & 'static str , # [doc = " How many non-ack-eliciting packets have been sent."] outstanding_non_ack_eliciting : usize , pub congestion : Congestion , # [doc = " A resusable list of acks."] newly_acked : Vec < Acked > , }
};
}
