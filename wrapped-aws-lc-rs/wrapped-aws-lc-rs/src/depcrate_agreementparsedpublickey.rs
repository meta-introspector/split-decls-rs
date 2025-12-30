// Generated macro for ParsedPublicKey (struct)
macro_rules! Depcrate_agreementParsedPublicKey {
() => {
// Module: crate::agreement
// Provides: {"ParsedPublicKey"}
// Dependencies: {}
# [doc = " A parsed public key for key agreement."] # [doc = ""] # [doc = " This represents a public key that has been successfully parsed and validated"] # [doc = " from its encoded form. The key can be used with the `agree` function to"] # [doc = " perform key agreement operations."] # [derive (Debug , Clone)] pub struct ParsedPublicKey { format : ParsedPublicKeyFormat , nid : i32 , key : LcPtr < EVP_PKEY > , bytes : Box < [u8] > , }
};
}
