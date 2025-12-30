// Generated macro for impl_62 (impl)
macro_rules! Depcrate_cipher_aes_cbcimpl_62 {
() => {
// Module: crate::cipher::aes_cbc
// Provides: {"impl_62"}
// Dependencies: {}
impl BlockCipher for Aes256Cbc { type Key = [u8 ; 32] ; type Nonce = [u8 ; 16] ; fn new_encrypt (key : & Self :: Key , nonce : & Self :: Nonce) -> Self { Self (Cipher :: new (key , nonce , CipherInitPurpose :: Encrypt)) } fn new_decrypt (key : & Self :: Key , nonce : & Self :: Nonce) -> Self { Self (Cipher :: new (key , nonce , CipherInitPurpose :: Decrypt)) } fn encrypt_padded (self , buffer : & [u8]) -> Result < Vec < u8 > , CipherError > { self . 0 . encrypt (buffer) } fn decrypt_padded (self , buffer : & [u8]) -> Result < Vec < u8 > , CipherError > { self . 0 . decrypt (buffer) } }
};
}
