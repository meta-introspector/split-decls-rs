// Generated macro for SigningKey (struct)
macro_rules! Depcrate_signing_keySigningKey {
() => {
// Module: crate::signing_key
// Provides: {"SigningKey"}
// Dependencies: {}
# [doc = " DSA private key."] # [doc = ""] # [doc = " The [`(try_)sign_digest_with_rng`](::signature::RandomizedDigestSigner) API uses regular non-deterministic signatures,"] # [doc = " while the [`(try_)sign_digest`](::signature::DigestSigner) API uses deterministic signatures as described in RFC 6979"] # [derive (Clone , PartialEq)] # [must_use] pub struct SigningKey { # [doc = " Public key"] verifying_key : VerifyingKey , # [doc = " Private component x"] x : Zeroizing < NonZero < BoxedUint > > , }
};
}
