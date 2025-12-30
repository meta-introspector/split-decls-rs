// Generated macro for Error (enum)
macro_rules! Depcrate_builderError {
() => {
// Module: crate::builder
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Debug)] # [non_exhaustive] pub enum Error { # [doc = " ASN.1 DER-related errors."] Asn1 (der :: Error) , # [doc = " Public key errors propagated from the [`spki::Error`] type."] PublicKey (spki :: Error) , # [doc = " RNG error."] Rng , # [doc = " Signing error propagated for the [`signature::Signer`] type."] Signature (signature :: Error) , # [doc = " Builder no table to build, because the struct is not properly configured"] Builder (String) , }
};
}
