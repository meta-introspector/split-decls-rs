// Generated macro for pkt_num_len (function)
macro_rules! Depcrate_packetpkt_num_len {
() => {
// Module: crate::packet
// Provides: {"pkt_num_len"}
// Dependencies: {}
pub fn pkt_num_len (pn : u64 , largest_acked : u64) -> usize { let num_unacked : u64 = pn . saturating_sub (largest_acked) + 1 ; let min_bits = u64 :: BITS - num_unacked . leading_zeros () + 1 ; min_bits . div_ceil (8) as usize }
};
}
