// Generated macro for Algorithm (struct)
macro_rules! Depcrate_pbes1Algorithm {
() => {
// Module: crate::pbes1
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " Password-Based Encryption Scheme 1 algorithms as defined in [RFC 8018 Appendix A.C]."] # [doc = ""] # [doc = " ```text"] # [doc = " PBES1Algorithms ALGORITHM-IDENTIFIER ::= {"] # [doc = "    {PBEParameter IDENTIFIED BY pbeWithMD2AndDES-CBC}  |"] # [doc = "    {PBEParameter IDENTIFIED BY pbeWithMD2AndRC2-CBC}  |"] # [doc = "    {PBEParameter IDENTIFIED BY pbeWithMD5AndDES-CBC}  |"] # [doc = "    {PBEParameter IDENTIFIED BY pbeWithMD5AndRC2-CBC}  |"] # [doc = "    {PBEParameter IDENTIFIED BY pbeWithSHA1AndDES-CBC} |"] # [doc = "    {PBEParameter IDENTIFIED BY pbeWithSHA1AndRC2-CBC},"] # [doc = "    ..."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8018 Appendix A.C]: https://datatracker.ietf.org/doc/html/rfc8018#appendix-C"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Algorithm { # [doc = " Encryption scheme."] pub encryption : EncryptionScheme , # [doc = " Scheme parameters."] pub parameters : Parameters , }
};
}
