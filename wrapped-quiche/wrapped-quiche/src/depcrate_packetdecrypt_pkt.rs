// Generated macro for decrypt_pkt (function)
macro_rules! Depcrate_packetdecrypt_pkt {
() => {
// Module: crate::packet
// Provides: {"decrypt_pkt"}
// Dependencies: {}
pub fn decrypt_pkt < 'a > (b : & 'a mut octets :: OctetsMut , pn : u64 , pn_len : usize , payload_len : usize , aead : & crypto :: Open ,) -> Result < octets :: Octets < 'a > > { let payload_offset = b . off () ; let (header , mut payload) = b . split_at (payload_offset) ? ; let payload_len = payload_len . checked_sub (pn_len) . ok_or (Error :: InvalidPacket) ? ; let mut ciphertext = payload . peek_bytes_mut (payload_len) ? ; let payload_len = aead . open_with_u64_counter (pn , header . as_ref () , ciphertext . as_mut ()) ? ; Ok (b . get_bytes (payload_len) ?) }
};
}
