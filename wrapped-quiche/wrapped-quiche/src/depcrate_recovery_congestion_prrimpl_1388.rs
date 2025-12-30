// Generated macro for impl_1388 (impl)
macro_rules! Depcrate_recovery_congestion_prrimpl_1388 {
() => {
// Module: crate::recovery::congestion::prr
// Provides: {"impl_1388"}
// Dependencies: {}
impl PRR { pub fn on_packet_sent (& mut self , sent_bytes : usize) { self . prr_out += sent_bytes ; self . snd_cnt = self . snd_cnt . saturating_sub (sent_bytes) ; } pub fn congestion_event (& mut self , bytes_in_flight : usize) { self . prr_delivered = 0 ; self . recoverfs = bytes_in_flight ; self . prr_out = 0 ; self . snd_cnt = 0 ; } pub fn on_packet_acked (& mut self , delivered_data : usize , pipe : usize , ssthresh : usize , max_datagram_size : usize ,) { self . prr_delivered += delivered_data ; self . snd_cnt = if pipe > ssthresh { if self . recoverfs > 0 { (self . prr_delivered * ssthresh) . div_ceil (self . recoverfs) . saturating_sub (self . prr_out) } else { 0 } } else { let limit = cmp :: max (self . prr_delivered . saturating_sub (self . prr_out) , delivered_data ,) + max_datagram_size ; cmp :: min (ssthresh - pipe , limit) } ; self . snd_cnt = cmp :: max (self . snd_cnt , 0) ; } }
};
}
