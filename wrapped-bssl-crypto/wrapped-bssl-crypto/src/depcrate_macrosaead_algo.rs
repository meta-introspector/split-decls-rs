// Generated macro for aead_algo (macro)
macro_rules! Depcrate_macrosaead_algo {
() => {
// Module: crate::macros
// Provides: {"aead_algo"}
// Dependencies: {}
macro_rules ! aead_algo { ($ name : ident , $ evp_md : ident , $ key_len : expr , $ nonce_len : expr , $ tag_len : expr) => { impl $ name { # [doc = " Create a new AEAD context for the given key."] pub fn new (key : & [u8 ; $ key_len]) -> Self { unsafe { $ name (EvpAead :: new (key , { bssl_sys ::$ evp_md () })) } } } impl Aead for $ name { type Tag = [u8 ; $ tag_len] ; type Nonce = [u8 ; $ nonce_len] ; fn seal (& self , nonce : & Self :: Nonce , plaintext : & [u8] , ad : & [u8]) -> Vec < u8 > { self . 0 . seal (nonce , plaintext , ad) } fn seal_in_place (& self , nonce : & Self :: Nonce , plaintext : & mut [u8] , ad : & [u8] ,) -> Self :: Tag { self . 0 . seal_in_place (nonce , plaintext , ad) } fn open (& self , nonce : & Self :: Nonce , ciphertext : & [u8] , ad : & [u8]) -> Option < Vec < u8 >> { self . 0 . open (nonce , ciphertext , ad) } fn open_in_place (& self , nonce : & Self :: Nonce , ciphertext : & mut [u8] , tag : & Self :: Tag , ad : & [u8] ,) -> Result < () , InvalidCiphertext > { self . 0 . open_in_place (nonce , ciphertext , tag , ad) } } } ; }
};
}
