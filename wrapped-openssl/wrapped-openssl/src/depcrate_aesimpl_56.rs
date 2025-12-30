// Generated macro for impl_56 (impl)
macro_rules! Depcrate_aesimpl_56 {
() => {
// Module: crate::aes
// Provides: {"impl_56"}
// Dependencies: {}
impl AesKey { # [doc = " Prepares a key for encryption."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Returns an error if the key is not 128, 192, or 256 bits."] # [corresponds (AES_set_encrypt_key)] pub fn new_encrypt (key : & [u8]) -> Result < AesKey , KeyError > { unsafe { assert ! (key . len () <= c_int :: MAX as usize / 8) ; let mut aes_key = MaybeUninit :: uninit () ; let r = ffi :: AES_set_encrypt_key (key . as_ptr () as * const _ , key . len () as AesBitType * 8 , aes_key . as_mut_ptr () ,) ; if r == 0 { Ok (AesKey (aes_key . assume_init ())) } else { Err (KeyError (())) } } } # [doc = " Prepares a key for decryption."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Returns an error if the key is not 128, 192, or 256 bits."] # [corresponds (AES_set_decrypt_key)] pub fn new_decrypt (key : & [u8]) -> Result < AesKey , KeyError > { unsafe { assert ! (key . len () <= c_int :: MAX as usize / 8) ; let mut aes_key = MaybeUninit :: uninit () ; let r = ffi :: AES_set_decrypt_key (key . as_ptr () as * const _ , key . len () as AesBitType * 8 , aes_key . as_mut_ptr () ,) ; if r == 0 { Ok (AesKey (aes_key . assume_init ())) } else { Err (KeyError (())) } } } }
};
}
