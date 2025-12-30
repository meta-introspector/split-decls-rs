// Generated macro for encrypt_hdr (function)
macro_rules! Depcrate_packetencrypt_hdr {
() => {
// Module: crate::packet
// Provides: {"encrypt_hdr"}
// Dependencies: {}
pub fn encrypt_hdr (b : & mut octets :: OctetsMut , pn_len : usize , payload : & [u8] , aead : & crypto :: Seal ,) -> Result < () > { let sample = & payload [MAX_PKT_NUM_LEN - pn_len .. SAMPLE_LEN + (MAX_PKT_NUM_LEN - pn_len)] ; let mask = aead . new_mask (sample) ? ; let (mut first , mut rest) = b . split_at (1) ? ; let first = first . as_mut () ; if Header :: is_long (first [0]) { first [0] ^= mask [0] & 0x0f ; } else { first [0] ^= mask [0] & 0x1f ; } let pn_buf = rest . slice_last (pn_len) ? ; for i in 0 .. pn_len { pn_buf [i] ^= mask [i + 1] ; } Ok (()) }
};
}
