// Generated macro for NistP256 (struct)
macro_rules! DepcrateNistP256 {
() => {
// Module: crate
// Provides: {"NistP256"}
// Dependencies: {}
# [doc = " NIST P-256 elliptic curve."] # [doc = ""] # [doc = " This curve is also known as prime256v1 (ANSI X9.62) and secp256r1 (SECG)"] # [doc = " and is specified in [NIST SP 800-186]:"] # [doc = " Recommendations for Discrete Logarithm-based Cryptography:"] # [doc = " Elliptic Curve Domain Parameters."] # [doc = ""] # [doc = " It's included in the US National Security Agency's \"Suite B\" and is widely"] # [doc = " used in protocols like TLS and the associated X.509 PKI."] # [doc = ""] # [doc = " Its equation is `y² = x³ - 3x + b` over a ~256-bit prime field where `b` is"] # [doc = " the \"verifiably random\"† constant:"] # [doc = ""] # [doc = " ```text"] # [doc = " b = 41058363725152142129326129780047268409114441015993725554835256314039467401291"] # [doc = " ```"] # [doc = ""] # [doc = " † *NOTE: the specific origins of this constant have never been fully disclosed"] # [doc = "   (it is the SHA-1 digest of an unknown NSA-selected constant)*"] # [doc = ""] # [doc = " [NIST SP 800-186]: https://csrc.nist.gov/publications/detail/sp/800-186/final"] # [derive (Copy , Clone , Debug , Default , Eq , PartialEq , PartialOrd , Ord)] pub struct NistP256 ;
};
}
