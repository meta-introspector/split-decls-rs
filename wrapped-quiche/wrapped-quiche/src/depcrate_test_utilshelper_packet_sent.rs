// Generated macro for helper_packet_sent (function)
macro_rules! Depcrate_test_utilshelper_packet_sent {
() => {
// Module: crate::test_utils
// Provides: {"helper_packet_sent"}
// Dependencies: {}
pub fn helper_packet_sent (pkt_num : u64 , now : Instant , size : usize) -> Sent { Sent { pkt_num , frames : smallvec ! [] , time_sent : now , time_acked : None , time_lost : None , size , ack_eliciting : true , in_flight : true , delivered : 0 , delivered_time : now , first_sent_time : now , is_app_limited : false , tx_in_flight : 0 , lost : 0 , has_data : true , is_pmtud_probe : false , } }
};
}
