// Generated macro for impl_38 (impl)
macro_rules! Depcrate_aesimpl_38 {
() => {
// Module: crate::aes
// Provides: {"impl_38"}
// Dependencies: {}
impl DecryptKey { # [doc = " Initializes a decryption key from an appropriately sized array of bytes for AES-128 operations."] pub fn new_128 (key : & [u8 ; 16]) -> DecryptKey { new_decrypt_key (key . as_slice ()) } # [doc = " Initializes a decryption key from an appropriately sized array of bytes for AES-256 operations."] pub fn new_256 (key : & [u8 ; 32]) -> DecryptKey { new_decrypt_key (key . as_slice ()) } # [doc = " Return the decrypted version of the given block."] pub fn decrypt (& self , block : & Block) -> Block { let mut ret = * block ; self . decrypt_in_place (& mut ret) ; ret } # [doc = " Replace `block` with its decrypted version."] pub fn decrypt_in_place (& self , block : & mut Block) { unsafe { bssl_sys :: AES_decrypt (block . as_ffi_ptr () , block . as_mut_ffi_ptr () , & self . 0) } } }
};
}
