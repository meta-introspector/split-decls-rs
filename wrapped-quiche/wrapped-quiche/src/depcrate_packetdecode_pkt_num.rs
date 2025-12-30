// Generated macro for decode_pkt_num (function)
macro_rules! Depcrate_packetdecode_pkt_num {
() => {
// Module: crate::packet
// Provides: {"decode_pkt_num"}
// Dependencies: {}
pub fn decode_pkt_num (largest_pn : u64 , truncated_pn : u64 , pn_len : usize) -> u64 { let pn_nbits = pn_len * 8 ; let expected_pn = largest_pn + 1 ; let pn_win = 1 << pn_nbits ; let pn_hwin = pn_win / 2 ; let pn_mask = pn_win - 1 ; let candidate_pn = (expected_pn & ! pn_mask) | truncated_pn ; if candidate_pn + pn_hwin <= expected_pn && candidate_pn < (1 << 62) - pn_win { return candidate_pn + pn_win ; } if candidate_pn > expected_pn + pn_hwin && candidate_pn >= pn_win { return candidate_pn - pn_win ; } candidate_pn }
};
}
