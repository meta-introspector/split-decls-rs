// Generated macro for hash_pubkey_to_g2 (function)
macro_rules! Depcrate_hashhash_pubkey_to_g2 {
() => {
// Module: crate::hash
// Provides: {"hash_pubkey_to_g2"}
// Dependencies: {}
# [doc = " Hash a pubkey to a G2 point"] pub (crate) fn hash_pubkey_to_g2 (public_key : & PubkeyProjective , payload : Option < & [u8] > ,) -> G2Projective { if let Some (bytes) = payload { G2Projective :: hash_to_curve (bytes , POP_DST , & []) } else { let public_key_bytes = public_key . 0 . to_compressed () ; G2Projective :: hash_to_curve (& public_key_bytes , POP_DST , & []) } }
};
}
