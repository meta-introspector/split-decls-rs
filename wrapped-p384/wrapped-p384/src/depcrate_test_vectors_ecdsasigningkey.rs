// Generated macro for SigningKey (type)
macro_rules! Depcrate_test_vectors_ecdsaSigningKey {
() => {
// Module: crate::test_vectors::ecdsa
// Provides: {"SigningKey"}
// Dependencies: {}
# [doc = " ECDSA/P-384 signing key"] # [cfg (feature = "ecdsa")] pub type SigningKey = ecdsa_core :: SigningKey < NistP384 > ;
};
}
