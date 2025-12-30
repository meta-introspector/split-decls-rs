// Generated macro for decrypt_hdr (function)
macro_rules! Depcrate_packetdecrypt_hdr {
() => {
// Module: crate::packet
// Provides: {"decrypt_hdr"}
// Dependencies: {}
pub fn decrypt_hdr (b : & mut octets :: OctetsMut , hdr : & mut Header , aead : & crypto :: Open ,) -> Result < () > { let mut first = { let (first_buf , _) = b . split_at (1) ? ; first_buf . as_ref () [0] } ; let mut pn_and_sample = b . peek_bytes_mut (MAX_PKT_NUM_LEN + SAMPLE_LEN) ? ; let (mut ciphertext , sample) = pn_and_sample . split_at (MAX_PKT_NUM_LEN) ? ; let ciphertext = ciphertext . as_mut () ; let mask = aead . new_mask (sample . as_ref ()) ? ; if Header :: is_long (first) { first ^= mask [0] & 0x0f ; } else { first ^= mask [0] & 0x1f ; } let pn_len = usize :: from ((first & PKT_NUM_MASK) + 1) ; let ciphertext = & mut ciphertext [.. pn_len] ; for i in 0 .. pn_len { ciphertext [i] ^= mask [i + 1] ; } let pn = match pn_len { 1 => u64 :: from (b . get_u8 () ?) , 2 => u64 :: from (b . get_u16 () ?) , 3 => u64 :: from (b . get_u24 () ?) , 4 => u64 :: from (b . get_u32 () ?) , _ => return Err (Error :: InvalidPacket) , } ; let (mut first_buf , _) = b . split_at (1) ? ; first_buf . as_mut () [0] = first ; hdr . pkt_num = pn ; hdr . pkt_num_len = pn_len ; if hdr . ty == Type :: Short { hdr . key_phase = (first & KEY_PHASE_BIT) != 0 ; } Ok (()) }
};
}
