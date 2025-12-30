// Generated macro for encrypt_pkt (function)
macro_rules! Depcrate_packetencrypt_pkt {
() => {
// Module: crate::packet
// Provides: {"encrypt_pkt"}
// Dependencies: {}
pub fn encrypt_pkt (b : & mut octets :: OctetsMut , pn : u64 , pn_len : usize , payload_len : usize , payload_offset : usize , extra_in : Option < & [u8] > , aead : & crypto :: Seal ,) -> Result < usize > { let (mut header , mut payload) = b . split_at (payload_offset) ? ; let ciphertext_len = aead . seal_with_u64_counter (pn , header . as_ref () , payload . as_mut () , payload_len , extra_in ,) ? ; encrypt_hdr (& mut header , pn_len , payload . as_ref () , aead) ? ; Ok (payload_offset + ciphertext_len) }
};
}
