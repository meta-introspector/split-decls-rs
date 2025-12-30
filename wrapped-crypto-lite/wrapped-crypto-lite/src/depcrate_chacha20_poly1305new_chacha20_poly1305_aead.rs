// Generated macro for new_chacha20_poly1305_aead (function)
macro_rules! Depcrate_chacha20_poly1305new_chacha20_poly1305_aead {
() => {
// Module: crate::chacha20_poly1305
// Provides: {"new_chacha20_poly1305_aead"}
// Dependencies: {}
# [allow (clippy :: missing_panics_doc)] fn new_chacha20_poly1305_aead (key : & Secret32 , nonce : & Secret12) -> (ChaCha20 , Poly1305) { let mut cipher = ChaCha20 :: new (key , nonce) ; let mut mac_key_bytes = [0u8 ; 16] ; cipher . apply (& mut mac_key_bytes) . unwrap () ; let mac_key = Secret16 (mac_key_bytes) ; let mut mac_nonce_bytes = [0u8 ; 16] ; cipher . apply (& mut mac_nonce_bytes) . unwrap () ; let mac_nonce = Secret16 (mac_nonce_bytes) ; let mac = Poly1305 :: new (& mac_key , & mac_nonce) ; cipher . seek_to (ChaCha20 :: BLOCK_SIZE as u64) . unwrap () ; (cipher , mac) }
};
}
