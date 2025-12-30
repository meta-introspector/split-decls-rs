// Generated macro for SigningKey (type)
macro_rules! Depcrate_ecdsaSigningKey {
() => {
// Module: crate::ecdsa
// Provides: {"SigningKey"}
// Dependencies: {}
# [doc = " ECDSA/secp256k1 signing key"] # [cfg (feature = "ecdsa")] pub type SigningKey = ecdsa_core :: SigningKey < Secp256k1 > ;
};
}
