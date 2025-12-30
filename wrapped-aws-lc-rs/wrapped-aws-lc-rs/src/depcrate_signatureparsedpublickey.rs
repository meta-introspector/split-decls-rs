// Generated macro for ParsedPublicKey (struct)
macro_rules! Depcrate_signatureParsedPublicKey {
() => {
// Module: crate::signature
// Provides: {"ParsedPublicKey"}
// Dependencies: {}
# [doc = " A parsed public key for signature verification."] # [doc = ""] # [doc = " A `ParsedPublicKey` can be created in two ways:"] # [doc = " - Directly from public key bytes using [`ParsedPublicKey::new`]"] # [doc = " - By parsing an `UnparsedPublicKey` using [`UnparsedPublicKey::parse`]"] # [doc = ""] # [doc = " This pre-validates the public key format and stores the parsed key material,"] # [doc = " allowing for more efficient signature verification operations compared to"] # [doc = " parsing the key on each verification."] # [doc = ""] # [doc = " See the [`crate::signature`] module-level documentation for examples."] # [derive (Clone)] pub struct ParsedPublicKey { algorithm : & 'static dyn VerificationAlgorithm , parsed_algorithm : & 'static dyn ParsedVerificationAlgorithm , key : LcPtr < EVP_PKEY > , bytes : Box < [u8] > , }
};
}
