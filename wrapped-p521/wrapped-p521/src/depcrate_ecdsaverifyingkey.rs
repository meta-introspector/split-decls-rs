// Generated macro for VerifyingKey (type)
macro_rules! Depcrate_ecdsaVerifyingKey {
() => {
// Module: crate::ecdsa
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [doc = " ECDSA/P-521 verification key (i.e. public key)"] # [cfg (feature = "ecdsa")] pub type VerifyingKey = ecdsa_core :: VerifyingKey < NistP521 > ;
};
}
