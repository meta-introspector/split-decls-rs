// Generated macro for DHKEM_X25519_HKDF_SHA256_AES_128 (static)
macro_rules! Depcrate_hpkeDHKEM_X25519_HKDF_SHA256_AES_128 {
() => {
// Module: crate::hpke
// Provides: {"DHKEM_X25519_HKDF_SHA256_AES_128"}
// Dependencies: {}
pub static DHKEM_X25519_HKDF_SHA256_AES_128 : & HpkeRs = & HpkeRs (HpkeSuite { kem : HpkeKemId :: DHKEM_X25519_HKDF_SHA256 , sym : HpkeSymmetricCipherSuite { kdf_id : HpkeKdfId :: HKDF_SHA256 , aead_id : HpkeAeadId :: AES_128_GCM , } , }) ;
};
}
