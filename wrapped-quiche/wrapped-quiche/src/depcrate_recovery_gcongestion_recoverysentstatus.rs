// Generated macro for SentStatus (enum)
macro_rules! Depcrate_recovery_gcongestion_recoverySentStatus {
() => {
// Module: crate::recovery::gcongestion::recovery
// Provides: {"SentStatus"}
// Dependencies: {}
# [derive (Debug)] enum SentStatus { Sent { time_sent : Instant , ack_eliciting : bool , in_flight : bool , has_data : bool , is_pmtud_probe : bool , sent_bytes : usize , frames : SmallVec < [frame :: Frame ; 1] > , } , Acked , Lost , }
};
}
