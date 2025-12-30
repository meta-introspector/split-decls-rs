// Generated macro for DHKEM_P256_HKDF_SHA256_AES_256 (static)
macro_rules! Depcrate_hpkeDHKEM_P256_HKDF_SHA256_AES_256 {
() => {
// Module: crate::hpke
// Provides: {"DHKEM_P256_HKDF_SHA256_AES_256"}
// Dependencies: {}
pub static DHKEM_P256_HKDF_SHA256_AES_256 : & HpkeRs = & HpkeRs (HpkeSuite { kem : HpkeKemId :: DHKEM_P256_HKDF_SHA256 , sym : HpkeSymmetricCipherSuite { kdf_id : HpkeKdfId :: HKDF_SHA256 , aead_id : HpkeAeadId :: AES_256_GCM , } , }) ;
};
}
