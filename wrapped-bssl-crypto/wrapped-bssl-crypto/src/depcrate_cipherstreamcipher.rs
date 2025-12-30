// Generated macro for StreamCipher (trait)
macro_rules! Depcrate_cipherStreamCipher {
() => {
// Module: crate::cipher
// Provides: {"StreamCipher"}
// Dependencies: {}
# [doc = " Synchronous stream cipher trait."] pub trait StreamCipher { # [doc = " The byte array key type which specifies the size of the key used to instantiate the cipher."] type Key : AsRef < [u8] > ; # [doc = " The byte array nonce type which specifies the size of the nonce used in the cipher"] # [doc = " operations."] type Nonce : AsRef < [u8] > ; # [doc = " Instantiate a new instance of a stream cipher from a `key` and `iv`."] fn new (key : & Self :: Key , iv : & Self :: Nonce) -> Self ; # [doc = " Applies the cipher keystream to `buffer` in place, returning CipherError on an unsuccessful"] # [doc = " operation."] fn apply_keystream (& mut self , buffer : & mut [u8]) -> Result < () , CipherError > ; }
};
}
