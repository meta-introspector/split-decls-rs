// Generated macro for GRecovery (struct)
macro_rules! Depcrate_recovery_gcongestion_recoveryGRecovery {
() => {
// Module: crate::recovery::gcongestion::recovery
// Provides: {"GRecovery"}
// Dependencies: {}
pub struct GRecovery { epochs : [RecoveryEpoch ; packet :: Epoch :: count ()] , loss_timer : LossDetectionTimer , pto_count : u32 , rtt_stats : RttStats , recovery_stats : RecoveryStats , pub lost_count : usize , pub lost_spurious_count : usize , loss_thresh : LossThreshold , bytes_in_flight : BytesInFlight , bytes_sent : usize , pub bytes_lost : u64 , max_datagram_size : usize , # [cfg (feature = "qlog")] qlog_metrics : QlogMetrics , # [cfg (feature = "qlog")] qlog_prev_cc_state : & 'static str , # [doc = " How many non-ack-eliciting packets have been sent."] outstanding_non_ack_eliciting : usize , # [doc = " A resusable list of acks."] newly_acked : Vec < Acked > , # [doc = " A [`Vec`] that can be reused for calls of"] # [doc = " [`Self::detect_and_remove_lost_packets`] to avoid allocations"] lost_reuse : Vec < Lost > , pacer : Pacer , }
};
}
