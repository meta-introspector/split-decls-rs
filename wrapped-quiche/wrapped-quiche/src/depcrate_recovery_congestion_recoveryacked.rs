// Generated macro for Acked (struct)
macro_rules! Depcrate_recovery_congestion_recoveryAcked {
() => {
// Module: crate::recovery::congestion::recovery
// Provides: {"Acked"}
// Dependencies: {}
# [derive (Clone)] pub struct Acked { pub pkt_num : u64 , pub time_sent : Instant , pub size : usize , pub rtt : Duration , pub delivered : usize , pub delivered_time : Instant , pub first_sent_time : Instant , pub is_app_limited : bool , }
};
}
