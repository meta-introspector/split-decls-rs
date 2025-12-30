// Generated macro for EncryptionScheme (enum)
macro_rules! DepcrateEncryptionScheme {
() => {
// Module: crate
// Provides: {"EncryptionScheme"}
// Dependencies: {}
# [doc = " Supported PKCS#5 password-based encryption schemes."] # [derive (Clone , Debug , Eq , PartialEq)] # [non_exhaustive] # [allow (clippy :: large_enum_variant)] pub enum EncryptionScheme { # [doc = " Password-Based Encryption Scheme 1 as defined in [RFC 8018 Section 6.1]."] # [doc = ""] # [doc = " [RFC 8018 Section 6.1]: https://tools.ietf.org/html/rfc8018#section-6.1"] Pbes1 (pbes1 :: Algorithm) , # [doc = " Password-Based Encryption Scheme 2 as defined in [RFC 8018 Section 6.2]."] # [doc = ""] # [doc = " [RFC 8018 Section 6.2]: https://tools.ietf.org/html/rfc8018#section-6.2"] Pbes2 (pbes2 :: Parameters) , }
};
}
