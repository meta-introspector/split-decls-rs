// Generated macro for impl_219 (impl)
macro_rules! Depcrate_cipherimpl_219 {
() => {
// Module: crate::cipher
// Provides: {"impl_219"}
// Dependencies: {}
impl CipherRef { # [doc = " Returns the cipher's Nid."] # [corresponds (EVP_CIPHER_nid)] pub fn nid (& self) -> Nid { let nid = unsafe { ffi :: EVP_CIPHER_nid (self . as_ptr ()) } ; Nid :: from_raw (nid) } # [doc = " Returns the length of keys used with this cipher."] # [corresponds (EVP_CIPHER_key_length)] pub fn key_length (& self) -> usize { unsafe { EVP_CIPHER_key_length (self . as_ptr ()) as usize } } # [doc = " Returns the length of the IV used with this cipher."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Ciphers that do not use an IV have an IV length of 0."] # [corresponds (EVP_CIPHER_iv_length)] pub fn iv_length (& self) -> usize { unsafe { EVP_CIPHER_iv_length (self . as_ptr ()) as usize } } # [doc = " Returns the block size of the cipher."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Stream ciphers have a block size of 1."] # [corresponds (EVP_CIPHER_block_size)] pub fn block_size (& self) -> usize { unsafe { EVP_CIPHER_block_size (self . as_ptr ()) as usize } } }
};
}
