// Generated macro for Pbkdf2Params (struct)
macro_rules! Depcrate_pbe_paramsPbkdf2Params {
() => {
// Module: crate::pbe_params
// Provides: {"Pbkdf2Params"}
// Dependencies: {}
# [doc = " Password-Based Key Derivation Scheme 2 parameters as defined in"] # [doc = " [RFC 8018 Appendix A.2]."] # [doc = ""] # [doc = " ```text"] # [doc = " PBKDF2-params ::= SEQUENCE {"] # [doc = "     salt CHOICE {"] # [doc = "         specified OCTET STRING,"] # [doc = "         otherSource AlgorithmIdentifier {{PBKDF2-SaltSources}}"] # [doc = "     },"] # [doc = "     iterationCount INTEGER (1..MAX),"] # [doc = "     keyLength INTEGER (1..MAX) OPTIONAL,"] # [doc = "     prf AlgorithmIdentifier {{PBKDF2-PRFs}} DEFAULT"] # [doc = "     algid-hmacWithSHA1 }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8018 Appendix A.2]: https://tools.ietf.org/html/rfc8018#appendix-A.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] pub struct Pbkdf2Params { # [doc = " PBKDF2 salt"] pub salt : OctetString , # [doc = " PBKDF2 iteration count"] pub iteration_count : u32 , # [doc = " PBKDF2 output length"] pub key_length : Option < u16 > , # [doc = " Pseudo-random function to use with PBKDF2"] pub prf : AlgorithmIdentifierOwned , }
};
}
