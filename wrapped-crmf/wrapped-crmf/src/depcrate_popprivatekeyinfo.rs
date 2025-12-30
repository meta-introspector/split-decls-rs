// Generated macro for PrivateKeyInfo (struct)
macro_rules! Depcrate_popPrivateKeyInfo {
() => {
// Module: crate::pop
// Provides: {"PrivateKeyInfo"}
// Dependencies: {}
# [doc = " The `PrivateKeyInfo` type is defined in [RFC 4211 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   PrivateKeyInfo ::= SEQUENCE {"] # [doc = "      version                   INTEGER,"] # [doc = "      privateKeyAlgorithm       AlgorithmIdentifier{PUBLIC-KEY, {...}},"] # [doc = "      privateKey                OCTET STRING,"] # [doc = "                --  Structure of public key is in PUBLIC-KEY.&PrivateKey"] # [doc = "      attributes                [0] IMPLICIT Attributes OPTIONAL"] # [doc = "   }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.2.1]: https://www.rfc-editor.org/rfc/rfc4211#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PrivateKeyInfo { pub version : u64 , pub priv_key_alg : AlgorithmIdentifierOwned , pub priv_key : OctetString , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub attrs : Option < Attributes > , }
};
}
