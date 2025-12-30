// Generated macro for impl_53 (impl)
macro_rules! Depcrate_cipher_aes_ctrimpl_53 {
() => {
// Module: crate::cipher::aes_ctr
// Provides: {"impl_53"}
// Dependencies: {}
impl StreamCipher for Aes256Ctr { type Key = [u8 ; 32] ; type Nonce = [u8 ; 16] ; # [doc = " Creates a new AES-256-CTR cipher instance from key material."] fn new (key : & Self :: Key , nonce : & Self :: Nonce) -> Self { Self (Cipher :: new (key , nonce , CipherInitPurpose :: Encrypt)) } # [doc = " Applies the keystream in-place, advancing the counter state appropriately."] fn apply_keystream (& mut self , buffer : & mut [u8]) -> Result < () , CipherError > { self . 0 . apply_keystream_in_place (buffer) } }
};
}
