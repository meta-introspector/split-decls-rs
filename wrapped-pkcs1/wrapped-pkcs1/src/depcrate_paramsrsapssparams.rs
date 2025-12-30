// Generated macro for RsaPssParams (struct)
macro_rules! Depcrate_paramsRsaPssParams {
() => {
// Module: crate::params
// Provides: {"RsaPssParams"}
// Dependencies: {}
# [doc = " PKCS#1 RSASSA-PSS parameters as defined in [RFC 8017 Appendix 2.3]"] # [doc = ""] # [doc = " ASN.1 structure containing a serialized RSASSA-PSS parameters:"] # [doc = " ```text"] # [doc = " RSASSA-PSS-params ::= SEQUENCE {"] # [doc = "     hashAlgorithm      [0] HashAlgorithm      DEFAULT sha1,"] # [doc = "     maskGenAlgorithm   [1] MaskGenAlgorithm   DEFAULT mgf1SHA1,"] # [doc = "     saltLength         [2] INTEGER            DEFAULT 20,"] # [doc = "     trailerField       [3] TrailerField       DEFAULT trailerFieldBC"] # [doc = " }"] # [doc = " HashAlgorithm ::= AlgorithmIdentifier"] # [doc = " MaskGenAlgorithm ::= AlgorithmIdentifier"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8017 Appendix 2.3]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.2.3"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct RsaPssParams < 'a > { # [doc = " Hash Algorithm"] pub hash : AlgorithmIdentifierRef < 'a > , # [doc = " Mask Generation Function (MGF)"] pub mask_gen : AlgorithmIdentifier < AlgorithmIdentifierRef < 'a > > , # [doc = " Salt length"] pub salt_len : u8 , # [doc = " Trailer field (i.e. [`TrailerField::BC`])"] pub trailer_field : TrailerField , }
};
}
