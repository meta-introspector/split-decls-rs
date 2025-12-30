// Generated macro for impl_36 (impl)
macro_rules! Depcrate_aesimpl_36 {
() => {
// Module: crate::aes
// Provides: {"impl_36"}
// Dependencies: {}
impl EncryptKey { # [doc = " Initializes an encryption key from an appropriately sized array of bytes"] pub fn new_128 (key : & [u8 ; 16]) -> Self { new_encrypt_key (key . as_slice ()) } # [doc = " Initializes an encryption key from an appropriately sized array of bytes"] pub fn new_256 (key : & [u8 ; 32]) -> Self { new_encrypt_key (key . as_slice ()) } # [doc = " Return the encrypted version of the given block."] pub fn encrypt (& self , block : & Block) -> Block { let mut ret = * block ; self . encrypt_in_place (& mut ret) ; ret } # [doc = " Replace `block` with its encrypted version."] pub fn encrypt_in_place (& self , block : & mut Block) { unsafe { bssl_sys :: AES_encrypt (block . as_ffi_ptr () , block . as_mut_ffi_ptr () , & self . 0) } } }
};
}
