// Generated macro for RevRepContent (struct)
macro_rules! Depcrate_revRevRepContent {
() => {
// Module: crate::rev
// Provides: {"RevRepContent"}
// Dependencies: {}
# [doc = " The `RevRepContent` type is defined in [RFC 4210 Section 5.3.10]."] # [doc = ""] # [doc = " ```text"] # [doc = "  RevRepContent ::= SEQUENCE {"] # [doc = "      status       SEQUENCE SIZE (1..MAX) OF PKIStatusInfo,"] # [doc = "      revCerts [0] SEQUENCE SIZE (1..MAX) OF CertId OPTIONAL,"] # [doc = "      crls     [1] SEQUENCE SIZE (1..MAX) OF CertificateList OPTIONAL"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.10]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.10"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct RevRepContent < 'a > { pub status : Vec < PkiStatusInfo < 'a > > , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , optional = "true")] pub rev_certs : Option < Vec < CertId > > , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , optional = "true")] pub crls : Option < Vec < CertificateList > > , }
};
}
