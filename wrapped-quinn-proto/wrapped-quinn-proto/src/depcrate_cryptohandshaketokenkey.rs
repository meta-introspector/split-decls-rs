// Generated macro for HandshakeTokenKey (trait)
macro_rules! Depcrate_cryptoHandshakeTokenKey {
() => {
// Module: crate::crypto
// Provides: {"HandshakeTokenKey"}
// Dependencies: {}
# [doc = " A pseudo random key for HKDF"] pub trait HandshakeTokenKey : Send + Sync { # [doc = " Derive AEAD using hkdf"] fn aead_from_hkdf (& self , random_bytes : & [u8]) -> Box < dyn AeadKey > ; }
};
}
