// Generated macro for print_sent_packet_stats (function)
macro_rules! Depcrate_reports_textprint_sent_packet_stats {
() => {
// Module: crate::reports::text
// Provides: {"print_sent_packet_stats"}
// Dependencies: {}
fn print_sent_packet_stats (data_store : & Datastore) { println ! ("### sent packets ###") ; for (pkt_space , pkts) in & data_store . packet_sent { println ! ("\t# packet space={:?}" , pkt_space) ; for (pkt_num , pkt_info) in pkts { let (length , payload_length) = match & pkt_info . raw { Some (raw) => (raw . length , raw . payload_length) , None => (None , None) , } ; println ! ("\tpkt_num={}, acked=TODO-unknown, length={:?}, payload_length={:?}, " , pkt_num , length , payload_length) ; } println ! () ; } }
};
}
