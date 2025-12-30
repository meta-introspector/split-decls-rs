// Generated macro for Parameters (struct)
macro_rules! Depcrate_pbes2Parameters {
() => {
// Module: crate::pbes2
// Provides: {"Parameters"}
// Dependencies: {}
# [doc = " Password-Based Encryption Scheme 2 parameters as defined in [RFC 8018 Appendix A.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "  PBES2-params ::= SEQUENCE {"] # [doc = "       keyDerivationFunc AlgorithmIdentifier {{PBES2-KDFs}},"] # [doc = "       encryptionScheme AlgorithmIdentifier {{PBES2-Encs}} }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8018 Appendix A.4]: https://tools.ietf.org/html/rfc8018#appendix-A.4"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Parameters { # [doc = " Key derivation function"] pub kdf : Kdf , # [doc = " Encryption scheme"] pub encryption : EncryptionScheme , }
};
}
