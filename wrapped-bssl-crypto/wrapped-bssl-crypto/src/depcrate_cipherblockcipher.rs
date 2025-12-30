// Generated macro for BlockCipher (trait)
macro_rules! Depcrate_cipherBlockCipher {
() => {
// Module: crate::cipher
// Provides: {"BlockCipher"}
// Dependencies: {}
# [doc = " Synchronous block cipher trait."] pub trait BlockCipher { # [doc = " The byte array key type which specifies the size of the key used to instantiate the cipher."] type Key : AsRef < [u8] > ; # [doc = " The byte array nonce type which specifies the size of the nonce used in the cipher"] # [doc = " operations."] type Nonce : AsRef < [u8] > ; # [doc = " Instantiate a new instance of a block cipher for encryption from a `key` and `iv`."] fn new_encrypt (key : & Self :: Key , iv : & Self :: Nonce) -> Self ; # [doc = " Instantiate a new instance of a block cipher for decryption from a `key` and `iv`."] fn new_decrypt (key : & Self :: Key , iv : & Self :: Nonce) -> Self ; # [doc = " Encrypts the given data in `buffer`, and returns the result (with padding) in a newly"] # [doc = " allocated vector, or a [`CipherError`] if the operation was unsuccessful."] fn encrypt_padded (self , buffer : & [u8]) -> Result < Vec < u8 > , CipherError > ; # [doc = " Decrypts the given data in a `buffer`, and returns the result (with padding removed) in a"] # [doc = " newly allocated vector, or a [`CipherError`] if the operation was unsuccessful."] fn decrypt_padded (self , buffer : & [u8]) -> Result < Vec < u8 > , CipherError > ; }
};
}
