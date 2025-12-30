// Generated macro for PublicKey (type)
macro_rules! DepcratePublicKey {
() => {
// Module: crate
// Provides: {"PublicKey"}
// Dependencies: {}
# [doc = " secp256k1 (K-256) public key."] # [cfg (feature = "arithmetic")] pub type PublicKey = elliptic_curve :: PublicKey < Secp256k1 > ;
};
}
