// Generated macro for impl_10 (impl)
macro_rules! Depcrate_aeadimpl_10 {
() => {
// Module: crate::aead
// Provides: {"impl_10"}
// Dependencies: {}
impl AeadCtxRef { pub fn encrypt (& self , data : & [u8] , nonce : & [u8] , ad : & [u8] , out : & mut [u8] ,) -> OpenSSLResult < () > { let mut out_len = out . len () ; unsafe { let res = ffi :: EVP_AEAD_CTX_seal (self . as_ptr () , out . as_mut_ptr () , & mut out_len , out . len () , nonce . as_ptr () , nonce . len () , data . as_ptr () , data . len () , ad . as_ptr () , ad . len () ,) ; cvt (res) ? ; } Ok (()) } pub fn decrypt (& self , data : & [u8] , nonce : & [u8] , ad : & [u8] , out : & mut [u8] ,) -> OpenSSLResult < () > { let mut out_len = out . len () ; unsafe { let res = ffi :: EVP_AEAD_CTX_open (self . as_ptr () , out . as_mut_ptr () , & mut out_len , out . len () , nonce . as_ptr () , nonce . len () , data . as_ptr () , data . len () , ad . as_ptr () , ad . len () ,) ; cvt (res) ? ; } Ok (()) } }
};
}
