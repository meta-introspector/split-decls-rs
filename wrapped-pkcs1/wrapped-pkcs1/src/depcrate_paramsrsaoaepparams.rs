// Generated macro for RsaOaepParams (struct)
macro_rules! Depcrate_paramsRsaOaepParams {
() => {
// Module: crate::params
// Provides: {"RsaOaepParams"}
// Dependencies: {}
# [doc = " PKCS#1 RSAES-OAEP parameters as defined in [RFC 8017 Appendix 2.1]"] # [doc = ""] # [doc = " ASN.1 structure containing a serialized RSAES-OAEP parameters:"] # [doc = " ```text"] # [doc = " RSAES-OAEP-params ::= SEQUENCE {"] # [doc = "     hashAlgorithm      [0] HashAlgorithm     DEFAULT sha1,"] # [doc = "     maskGenAlgorithm   [1] MaskGenAlgorithm  DEFAULT mgf1SHA1,"] # [doc = "     pSourceAlgorithm   [2] PSourceAlgorithm  DEFAULT pSpecifiedEmpty"] # [doc = " }"] # [doc = " HashAlgorithm ::= AlgorithmIdentifier"] # [doc = " MaskGenAlgorithm ::= AlgorithmIdentifier"] # [doc = " PSourceAlgorithm ::= AlgorithmIdentifier"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8017 Appendix 2.1]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.2.1"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct RsaOaepParams < 'a > { # [doc = " Hash Algorithm"] pub hash : AlgorithmIdentifierRef < 'a > , # [doc = " Mask Generation Function (MGF)"] pub mask_gen : AlgorithmIdentifier < AlgorithmIdentifierRef < 'a > > , # [doc = " The source (and possibly the value) of the label L"] pub p_source : AlgorithmIdentifierRef < 'a > , }
};
}
