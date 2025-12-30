// Generated macro for PublicKey (type)
macro_rules! DepcratePublicKey {
() => {
// Module: crate
// Provides: {"PublicKey"}
// Dependencies: {}
# [doc = " NIST P-384 public key."] # [cfg (feature = "arithmetic")] pub type PublicKey = elliptic_curve :: PublicKey < NistP384 > ;
};
}
