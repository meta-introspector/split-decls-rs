// Generated macro for impl_267 (impl)
macro_rules! Depcrate_slhdsaimpl_267 {
() => {
// Module: crate::slhdsa
// Provides: {"impl_267"}
// Dependencies: {}
impl PrivateKey { # [doc = " Generates a random public/private key pair."] pub fn generate () -> (PublicKey , Self) { let mut public_key = [0u8 ; PUBLIC_KEY_BYTES] ; let mut private_key = [0u8 ; PRIVATE_KEY_BYTES] ; unsafe { bssl_sys :: SLHDSA_SHA2_128S_generate_key (public_key . as_mut_ptr () , private_key . as_mut_ptr () ,) ; } (PublicKey (public_key) , Self (private_key)) } # [doc = " Derives the public key corresponding to this private key."] pub fn to_public_key (& self) -> PublicKey { let mut public_key = [0u8 ; PUBLIC_KEY_BYTES] ; unsafe { bssl_sys :: SLHDSA_SHA2_128S_public_from_private (public_key . as_mut_ptr () , self . 0 . as_ptr () ,) ; } PublicKey (public_key) } # [doc = " Signs a message using this private key."] pub fn sign (& self , msg : & [u8]) -> Vec < u8 > { # [allow (clippy :: expect_used)] self . sign_with_context (msg , & []) . expect ("Empty context should always succeed") } # [doc = " Signs a message using this private key and the given context."] # [doc = ""] # [doc = " This function returns None if `context` is longer than 255 bytes."] pub fn sign_with_context (& self , msg : & [u8] , context : & [u8]) -> Option < Vec < u8 > > { unsafe { with_output_vec_fallible (SIGNATURE_BYTES , | signature | { if bssl_sys :: SLHDSA_SHA2_128S_sign (signature , self . 0 . as_ptr () , msg . as_ffi_ptr () , msg . len () , context . as_ffi_ptr () , context . len () ,) == 1 { Some (SIGNATURE_BYTES) } else { None } }) } } }
};
}
