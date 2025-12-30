// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " ASN.1 DER-related errors."] Asn1 (der :: Error) , # [doc = " Errors relating to PKCS#5-encrypted keys."] # [cfg (feature = "pkcs5")] EncryptedPrivateKey (pkcs5 :: Error) , # [doc = " Malformed cryptographic key contained in a PKCS#8 document."] # [doc = ""] # [doc = " This is intended for relaying errors related to the raw data contained"] # [doc = " within [`PrivateKeyInfo::private_key`][`crate::PrivateKeyInfo::private_key`]"] # [doc = " or [`SubjectPublicKeyInfo::subject_public_key`][`crate::SubjectPublicKeyInfo::subject_public_key`]."] KeyMalformed , # [doc = " [`AlgorithmIdentifier::parameters`][`crate::AlgorithmIdentifierRef::parameters`]"] # [doc = " is malformed or otherwise encoded in an unexpected manner."] ParametersMalformed , # [doc = " Public key errors propagated from the [`spki::Error`] type."] PublicKey (spki :: Error) , }
};
}
