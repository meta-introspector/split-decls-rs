// Generated macro for OobCertHash (struct)
macro_rules! Depcrate_oobOobCertHash {
() => {
// Module: crate::oob
// Provides: {"OobCertHash"}
// Dependencies: {}
# [doc = " The `OOBCertHash` type is defined in [RFC 4210 Section 5.2.5]."] # [doc = ""] # [doc = " ```text"] # [doc = "  OOBCertHash ::= SEQUENCE {"] # [doc = "      hashAlg     [0] AlgorithmIdentifier{DIGEST-ALGORITHM, {...}}"] # [doc = "                          OPTIONAL,"] # [doc = "      certId      [1] CertId                  OPTIONAL,"] # [doc = "      hashVal         BIT STRING"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.5]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.5"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OobCertHash < P : Profile = Rfc5280 > { # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub hash_alg : Option < AlgorithmIdentifierOwned > , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub cert_id : Option < CertId < P > > , pub hash_val : BitString , }
};
}
