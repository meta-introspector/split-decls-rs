// Generated macro for Kem (enum)
macro_rules! Depcrate_hpkeKem {
() => {
// Module: crate::hpke
// Provides: {"Kem"}
// Dependencies: {}
# [doc = " Supported KEM algorithms with values detailed in RFC 9180."] # [derive (Clone , Copy)] pub enum Kem { # [doc = " KEM using DHKEM P-256 and HKDF-SHA256."] P256HkdfSha256 = 16 , # [doc = " KEM using DHKEM X25519 and HKDF-SHA256."] X25519HkdfSha256 = 32 , # [doc = " X-Wing hybrid KEM."] XWing = 25722 , # [doc = " ML-KEM-768."] MlKem768 = 65 , # [doc = " ML-KEM-1024."] MlKem1024 = 66 , }
};
}
