// Generated macro for KeyRecRepContent (struct)
macro_rules! Depcrate_certified_key_pairKeyRecRepContent {
() => {
// Module: crate::certified_key_pair
// Provides: {"KeyRecRepContent"}
// Dependencies: {}
# [doc = " The `KeyRecRepContent` type is defined in [RFC 4210 Section 5.3.8]"] # [doc = ""] # [doc = " ```text"] # [doc = "  KeyRecRepContent ::= SEQUENCE {"] # [doc = "      status                  PKIStatusInfo,"] # [doc = "      newSigCert          [0] CMPCertificate OPTIONAL,"] # [doc = "      caCerts             [1] SEQUENCE SIZE (1..MAX) OF"] # [doc = "                                       CMPCertificate OPTIONAL,"] # [doc = "      keyPairHist         [2] SEQUENCE SIZE (1..MAX) OF"] # [doc = "                                       CertifiedKeyPair OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.8]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.8"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct KeyRecRepContent < 'a > { pub status : PkiStatusInfo < 'a > , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub new_sig_cert : Option < CmpCertificate > , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub ca_certs : Option < Vec < CmpCertificate > > , # [asn1 (context_specific = "2" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub key_pair_hist : Option < Vec < CertifiedKeyPair > > , }
};
}
