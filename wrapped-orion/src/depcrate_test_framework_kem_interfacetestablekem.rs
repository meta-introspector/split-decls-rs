// Generated macro for TestableKem (trait)
macro_rules! Depcrate_test_framework_kem_interfaceTestableKem {
() => {
// Module: crate::test_framework::kem_interface
// Provides: {"TestableKem"}
// Dependencies: {}
pub trait TestableKem < K : PartialEq , C : PartialEq + AsRef < [u8] > > { fn keygen (seed : & [u8]) -> Result < (Vec < u8 > , Vec < u8 >) , UnknownCryptoError > ; fn ciphertext_from_bytes (b : & [u8]) -> Result < C , UnknownCryptoError > ; fn encap (ek : & [u8]) -> Result < (K , C) , UnknownCryptoError > ; fn decap (dk : & [u8] , c : & C) -> Result < K , UnknownCryptoError > ; }
};
}
