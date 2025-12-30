// Generated macro for impl_188 (impl)
macro_rules! Depcrate_hpkeimpl_188 {
() => {
// Module: crate::hpke
// Provides: {"impl_188"}
// Dependencies: {}
impl Aead { fn from_rfc_id (n : u16) -> Option < Aead > { let ret = match n { 1 => Aead :: Aes128Gcm , 2 => Aead :: Aes256Gcm , 3 => Aead :: Chacha20Poly1305 , _ => return None , } ; assert_eq ! (n , ret as u16) ; Some (ret) } fn as_ffi_ptr (& self) -> * const bssl_sys :: EVP_HPKE_AEAD { unsafe { match self { Aead :: Aes128Gcm => bssl_sys :: EVP_hpke_aes_128_gcm () , Aead :: Aes256Gcm => bssl_sys :: EVP_hpke_aes_256_gcm () , Aead :: Chacha20Poly1305 => bssl_sys :: EVP_hpke_chacha20_poly1305 () , } } } }
};
}
