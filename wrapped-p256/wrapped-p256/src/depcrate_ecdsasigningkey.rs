// Generated macro for SigningKey (type)
macro_rules! Depcrate_ecdsaSigningKey {
() => {
// Module: crate::ecdsa
// Provides: {"SigningKey"}
// Dependencies: {}
# [doc = " ECDSA/P-256 signing key"] # [cfg (feature = "ecdsa")] pub type SigningKey = ecdsa_core :: SigningKey < NistP256 > ;
};
}
