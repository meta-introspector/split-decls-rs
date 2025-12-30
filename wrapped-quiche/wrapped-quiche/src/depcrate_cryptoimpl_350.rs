// Generated macro for impl_350 (impl)
macro_rules! Depcrate_cryptoimpl_350 {
() => {
// Module: crate::crypto
// Provides: {"impl_350"}
// Dependencies: {}
impl Algorithm { fn get_evp_digest (self) -> * const EVP_MD { match self { Algorithm :: AES128_GCM => unsafe { EVP_sha256 () } , Algorithm :: AES256_GCM => unsafe { EVP_sha384 () } , Algorithm :: ChaCha20_Poly1305 => unsafe { EVP_sha256 () } , } } pub const fn key_len (self) -> usize { match self { Algorithm :: AES128_GCM => 16 , Algorithm :: AES256_GCM => 32 , Algorithm :: ChaCha20_Poly1305 => 32 , } } pub const fn tag_len (self) -> usize { if cfg ! (feature = "fuzzing") { return 16 ; } match self { Algorithm :: AES128_GCM => 16 , Algorithm :: AES256_GCM => 16 , Algorithm :: ChaCha20_Poly1305 => 16 , } } pub const fn nonce_len (self) -> usize { match self { Algorithm :: AES128_GCM => 12 , Algorithm :: AES256_GCM => 12 , Algorithm :: ChaCha20_Poly1305 => 12 , } } }
};
}
