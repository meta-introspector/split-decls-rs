// Generated macro for VerifyingKey (type)
macro_rules! Depcrate_ecdsaVerifyingKey {
() => {
// Module: crate::ecdsa
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [doc = " ECDSA/secp256k1 verification key (i.e. public key)"] # [cfg (feature = "ecdsa")] pub type VerifyingKey = ecdsa_core :: VerifyingKey < Secp256k1 > ;
};
}
