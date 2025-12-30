// Generated macro for Sent (struct)
macro_rules! Depcrate_recoverySent {
() => {
// Module: crate::recovery
// Provides: {"Sent"}
// Dependencies: {}
# [derive (Clone)] pub struct Sent { pub pkt_num : u64 , pub frames : SmallVec < [frame :: Frame ; 1] > , pub time_sent : Instant , pub time_acked : Option < Instant > , pub time_lost : Option < Instant > , pub size : usize , pub ack_eliciting : bool , pub in_flight : bool , pub delivered : usize , pub delivered_time : Instant , pub first_sent_time : Instant , pub is_app_limited : bool , pub tx_in_flight : usize , pub lost : u64 , pub has_data : bool , pub is_pmtud_probe : bool , }
};
}
