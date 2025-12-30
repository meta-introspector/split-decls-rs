// Generated macro for impl_405 (impl)
macro_rules! Depcrate_crypto_openssl_quictlsimpl_405 {
() => {
// Module: crate::crypto::openssl_quictls
// Provides: {"impl_405"}
// Dependencies: {}
impl HeaderProtectionKey { pub fn new (alg : Algorithm , hp_key : Vec < u8 >) -> Result < Self > { Ok (Self { ctx : make_evp_cipher_ctx_basic (alg , false , 1) ? , key : hp_key , }) } pub fn new_mask (& self , sample : & [u8]) -> Result < HeaderProtectionMask > { const PLAINTEXT : & [u8 ; 5] = & [0_u8 ; 5] ; let mut new_mask = HeaderProtectionMask :: default () ; let rc = unsafe { EVP_CipherInit_ex2 (self . ctx , std :: ptr :: null_mut () , self . key . as_ptr () , sample . as_ptr () , - 1 , std :: ptr :: null () ,) } ; if rc != 1 { return Err (Error :: CryptoFail) ; } let mut out_len : i32 = 0 ; let rc = unsafe { EVP_CipherUpdate (self . ctx , new_mask . as_mut_ptr () , & mut out_len , PLAINTEXT . as_ptr () , PLAINTEXT . len () as i32 ,) } ; if rc != 1 { return Err (Error :: CryptoFail) ; } ; let rc = unsafe { EVP_CipherFinal_ex (self . ctx , new_mask [out_len as usize ..] . as_mut_ptr () , & mut out_len ,) } ; if rc != 1 { return Err (Error :: CryptoFail) ; } Ok (new_mask) } }
};
}
