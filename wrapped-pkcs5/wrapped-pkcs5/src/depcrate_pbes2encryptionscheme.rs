// Generated macro for EncryptionScheme (enum)
macro_rules! Depcrate_pbes2EncryptionScheme {
() => {
// Module: crate::pbes2
// Provides: {"EncryptionScheme"}
// Dependencies: {}
# [doc = " Symmetric encryption scheme used by PBES2."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum EncryptionScheme { # [doc = " AES-128 in CBC mode"] Aes128Cbc { # [doc = " Initialization vector"] iv : [u8 ; AES_BLOCK_SIZE] , } , # [doc = " AES-192 in CBC mode"] Aes192Cbc { # [doc = " Initialization vector"] iv : [u8 ; AES_BLOCK_SIZE] , } , # [doc = " AES-256 in CBC mode"] Aes256Cbc { # [doc = " Initialization vector"] iv : [u8 ; AES_BLOCK_SIZE] , } , # [doc = " AES-128 in CBC mode"] Aes128Gcm { # [doc = " GCM nonce"] nonce : [u8 ; GCM_NONCE_SIZE] , } , # [doc = " AES-256 in GCM mode"] Aes256Gcm { # [doc = " GCM nonce"] nonce : [u8 ; GCM_NONCE_SIZE] , } , # [doc = " 3-Key Triple DES in CBC mode"] # [cfg (feature = "3des")] DesEde3Cbc { # [doc = " Initialisation vector"] iv : [u8 ; DES_BLOCK_SIZE] , } , # [doc = " DES in CBC mode"] # [cfg (feature = "des-insecure")] DesCbc { # [doc = " Initialisation vector"] iv : [u8 ; DES_BLOCK_SIZE] , } , }
};
}
