// Generated macro for compute_retry_integrity_tag (function)
macro_rules! Depcrate_packetcompute_retry_integrity_tag {
() => {
// Module: crate::packet
// Provides: {"compute_retry_integrity_tag"}
// Dependencies: {}
fn compute_retry_integrity_tag (b : & octets :: OctetsMut , odcid : & [u8] , version : u32 ,) -> Result < Vec < u8 > > { const KEY_LEN : usize = RETRY_AEAD_ALG . key_len () ; const TAG_LEN : usize = RETRY_AEAD_ALG . tag_len () ; const RETRY_INTEGRITY_KEY_V1 : [u8 ; KEY_LEN] = [0xbe , 0x0c , 0x69 , 0x0b , 0x9f , 0x66 , 0x57 , 0x5a , 0x1d , 0x76 , 0x6b , 0x54 , 0xe3 , 0x68 , 0xc8 , 0x4e ,] ; const RETRY_INTEGRITY_NONCE_V1 : [u8 ; crypto :: MAX_NONCE_LEN] = [0x46 , 0x15 , 0x99 , 0xd3 , 0x5d , 0x63 , 0x2b , 0xf2 , 0x23 , 0x98 , 0x25 , 0xbb ,] ; let (key , nonce) = match version { crate :: PROTOCOL_VERSION_V1 => (& RETRY_INTEGRITY_KEY_V1 , RETRY_INTEGRITY_NONCE_V1) , _ => (& RETRY_INTEGRITY_KEY_V1 , RETRY_INTEGRITY_NONCE_V1) , } ; let hdr_len = b . off () ; let mut pseudo = vec ! [0 ; 1 + odcid . len () + hdr_len] ; let mut pb = octets :: OctetsMut :: with_slice (& mut pseudo) ; pb . put_u8 (odcid . len () as u8) ? ; pb . put_bytes (odcid) ? ; pb . put_bytes (& b . buf () [.. hdr_len]) ? ; let key = crypto :: PacketKey :: new (RETRY_AEAD_ALG , key . to_vec () , nonce . to_vec () , crypto :: Seal :: ENCRYPT ,) ? ; let mut out_tag = vec ! [0_u8 ; TAG_LEN] ; let out_len = key . seal_with_u64_counter (0 , & pseudo , & mut out_tag , 0 , None) ? ; if out_len != out_tag . len () { return Err (Error :: CryptoFail) ; } Ok (out_tag) }
};
}
