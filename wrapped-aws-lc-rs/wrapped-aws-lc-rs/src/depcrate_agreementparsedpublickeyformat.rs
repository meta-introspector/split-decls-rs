// Generated macro for ParsedPublicKeyFormat (enum)
macro_rules! Depcrate_agreementParsedPublicKeyFormat {
() => {
// Module: crate::agreement
// Provides: {"ParsedPublicKeyFormat"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Eq , Debug)] # [doc = " The format of a parsed public key."] # [doc = ""] # [doc = " This is used to distinguish between different types of public key formats"] # [doc = " supported by *aws-lc-rs*."] # [non_exhaustive] pub enum ParsedPublicKeyFormat { # [doc = " The key is in an X.509 SubjectPublicKeyInfo format."] X509 , # [doc = " The key is in an uncompressed form (X9.62)."] Uncompressed , # [doc = " The key is in a compressed form (SEC 1: Elliptic Curve Cryptography, Version 2.0)."] Compressed , # [doc = " The key is in a hybrid form (SEC 1: Elliptic Curve Cryptography, Version 2.0)."] Hybrid , # [doc = " The key is in a raw form. (X25519 only)"] Raw , # [doc = " The key is in an unknown format."] Unknown , }
};
}
