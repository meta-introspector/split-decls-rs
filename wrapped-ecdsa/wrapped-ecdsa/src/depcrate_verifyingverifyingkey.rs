// Generated macro for VerifyingKey (struct)
macro_rules! Depcrate_verifyingVerifyingKey {
() => {
// Module: crate::verifying
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [doc = " ECDSA public key used for verifying signatures. Generic over prime order"] # [doc = " elliptic curves (e.g. NIST P-curves)."] # [doc = ""] # [doc = " Requires an [`elliptic_curve::CurveArithmetic`] impl on the curve."] # [doc = ""] # [doc = " ## Usage"] # [doc = ""] # [doc = " The [`signature`] crate defines the following traits which are the"] # [doc = " primary API for verifying:"] # [doc = ""] # [doc = " - [`Verifier`]: verify a message against a provided key and signature"] # [doc = " - [`DigestVerifier`]: verify a message [`Digest`] against a provided key and signature"] # [doc = " - [`PrehashVerifier`]: verify the low-level raw output bytes of a message digest"] # [doc = ""] # [doc = " See the [`p256` crate](https://docs.rs/p256/latest/p256/ecdsa/index.html)"] # [doc = " for examples of using this type with a concrete elliptic curve."] # [doc = ""] # [doc = " # `serde` support"] # [doc = ""] # [doc = " When the `serde` feature of this crate is enabled, it provides support for"] # [doc = " serializing and deserializing ECDSA signatures using the `Serialize` and"] # [doc = " `Deserialize` traits."] # [doc = ""] # [doc = " The serialization leverages the encoding used by the [`PublicKey`] type,"] # [doc = " which is a binary-oriented ASN.1 DER encoding."] # [derive (Clone , Debug)] pub struct VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , { pub (crate) inner : PublicKey < C > , }
};
}
