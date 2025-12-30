// Generated macro for impl_270 (impl)
macro_rules! Depcrate_slhdsaimpl_270 {
() => {
// Module: crate::slhdsa
// Provides: {"impl_270"}
// Dependencies: {}
impl PublicKey { # [doc = " Verifies a signature for a given message using this public key."] pub fn verify (& self , msg : & [u8] , signature : & [u8]) -> Result < () , InvalidSignatureError > { self . verify_with_context (msg , signature , & []) } # [doc = " Verifies a signature for a given message using this public key and the given context."] pub fn verify_with_context (& self , msg : & [u8] , signature : & [u8] , context : & [u8] ,) -> Result < () , InvalidSignatureError > { let ok = unsafe { bssl_sys :: SLHDSA_SHA2_128S_verify (signature . as_ffi_ptr () , signature . len () , self . 0 . as_ptr () , msg . as_ffi_ptr () , msg . len () , context . as_ffi_ptr () , context . len () ,) } ; if ok == 1 { Ok (()) } else { Err (InvalidSignatureError) } } }
};
}
